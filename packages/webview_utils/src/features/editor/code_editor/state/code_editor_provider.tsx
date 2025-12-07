import React, {
    ReactNode,
    createContext,
    useReducer,
    useContext,
    useEffect,
} from "react";
import {
    CodeEditorBaseKeymap,
    CodeEditorTheme,
    stringToCodeEditorTheme,
} from "../types/code_editor_types";
import { useLocalStorage } from "@/state/hooks/use_local_storage";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { SplitviewEditorWebviewActions, SplitviewEditorWebviewEvents, SplitviewEditorWebviewLocalStorageKeys } from "@/code_gen/typeshare/fluster_core_utilities";
import { CitationResultBuffer, MdxParsingResultBuffer, TagResultBuffer } from "@/code_gen/flat_buffer/mdx-serialization";

export interface CodeEditorState {
    keymap: string;
    baseKeymap: CodeEditorBaseKeymap;
    theme: CodeEditorTheme;
    citations: CitationResultBuffer[]
    tags: TagResultBuffer[]
    value: string;
    parsedValue: string | null;
    haveSetInitialValue: boolean;
}

const defaultInitialCodeEditorState: CodeEditorState = {
    baseKeymap: CodeEditorBaseKeymap.default,
    theme: CodeEditorTheme.dracula,
    keymap: "base",
    value: "",
    citations: [],
    tags: [],
    parsedValue: null,
    haveSetInitialValue: false,
};

export const CodeEditorContext = createContext<CodeEditorState>(
    defaultInitialCodeEditorState,
);

type CodeEditorContextActions =
    | {
        type: "setValue";
        payload: string;
    }
    | {
        type: "setTheme";
        payload: CodeEditorTheme;
    }
    | {
        type: "setBaseKeymap";
        payload: CodeEditorBaseKeymap;
    }
    | {
        type: "setKeymap";
        payload: string;
    }
    | {
        type: "setParsedEditorContentString",
        payload: string
    } | {
        type: "setInitialEditorValue";
        payload: string;
    }
    | {
        type: "setParsedEditorContent";
        payload: MdxParsingResultBuffer;
    };

export const CodeEditorDispatchContext = createContext<
    React.Dispatch<CodeEditorContextActions>
>(null!);

export const useCodeEditorContext = () => useContext(CodeEditorContext);
export const useCodeEditorDispatch = () =>
    useContext(CodeEditorDispatchContext);

export const CodeEditorContextReducer = (
    state: CodeEditorState,
    action: CodeEditorContextActions,
): CodeEditorState => {
    switch (action.type) {
        case "setValue": {
            return {
                ...state,
                value: action.payload,
            };
        }
        case "setTheme": {
            return {
                ...state,
                theme: action.payload,
            };
        }
        case "setKeymap": {
            return {
                ...state,
                keymap: action.payload,
            };
        }
        case "setBaseKeymap": {
            return {
                ...state,
                baseKeymap: action.payload,
            };
        }
        case "setInitialEditorValue": {
            if (state.haveSetInitialValue) {
                return state;
            }
            return {
                ...state,
                haveSetInitialValue: true,
                value: action.payload,
            };
        }
        case "setParsedEditorContentString": {
            return {
                ...state,
                parsedValue: action.payload
            }
        }
        case "setParsedEditorContent": {
            const content = action.payload.parsedContent()
            const citations: CitationResultBuffer[] = [];
            for (let i = 0; i < action.payload.citationsLength(); i++) {
                const cit = action.payload.citations(i)
                if (cit) {
                    citations.push()
                }
            }
            const tags: TagResultBuffer[] = []
            for (let i = 0; i < action.payload.tagsLength(); i++) {
                const tag = action.payload.tags(i)
                if (tag) {
                    tags.push()
                }
            }
            return {
                ...state,
                parsedValue: content ? content : state.parsedValue,
                citations,
                tags
            };
        }
        default: {
            return state;
        }
    }
};

CodeEditorContextReducer.displayName = "CodeEditorContextReducer";

interface CodeEditorProviderProps {
    children: ReactNode;
    initialValues?: Partial<CodeEditorState>;
    initialValueKey?: string;
}

export const CodeEditorProvider = ({
    children,
    initialValues,
    initialValueKey = SplitviewEditorWebviewLocalStorageKeys.InitialValue,
}: CodeEditorProviderProps) => {
    const [state, dispatch] = useReducer(
        CodeEditorContextReducer,
        initialValues
            ? { ...initialValues, ...defaultInitialCodeEditorState }
            : defaultInitialCodeEditorState,
    );

    const [editorKeymap] = useLocalStorage(SplitviewEditorWebviewLocalStorageKeys.EditorKeymap, "base", {
        deserializer(value) {
            return value;
        },
        serializer(value) {
            return value;
        },
        initializeWithValue: false,
    });
    useEffect(() => {
        dispatch({
            type: "setKeymap",
            payload: editorKeymap,
        });
    }, [editorKeymap]);

    useEventListener(SplitviewEditorWebviewEvents.SetEditorKeymap, (e) => {
        dispatch({
            type: "setKeymap",
            payload: e.detail,
        });
    });

    const [editorTheme] = useLocalStorage(SplitviewEditorWebviewLocalStorageKeys.CodeTheme, undefined, {
        deserializer(value) {
            return value;
        },
        serializer(value) {
            return value;
        },
        initializeWithValue: false,
    });
    const handleTheme = (t: string): void => {
        const payload = stringToCodeEditorTheme((t as string) ?? "dracula");
        dispatch({
            type: "setTheme",
            payload,
        });
    };
    useEffect(() => {
        handleTheme(editorTheme);
    }, [editorTheme]);
    useEventListener(SplitviewEditorWebviewEvents.SetCodeTheme, (e) => {
        handleTheme(e.detail);
    });

    const [initialValue] = useLocalStorage(initialValueKey, undefined, {
        deserializer(value) {
            return value;
        },
        serializer(value) {
            return value;
        },
        initializeWithValue: false,
    });
    useEffect(() => {
        if (!state.haveSetInitialValue && typeof initialValue === "string") {
            dispatch({
                type: "setInitialEditorValue",
                payload: initialValue,
            });
        }
    }, [initialValue, state.haveSetInitialValue]);

    useEventListener(SplitviewEditorWebviewEvents.SetParsedMdxContent, (e) => {
        dispatch({
            type: "setParsedEditorContent",
            payload: e.detail,
        });
    });

    useEventListener(SplitviewEditorWebviewEvents.SetParsedMdxContentString, (e) => {
        dispatch({
            type: "setParsedEditorContentString",
            payload: e.detail
        })
    })

    useEffect(() => {
        if (state.parsedValue === null) {
            sendToSwift(SplitviewEditorWebviewActions.RequestSplitviewEditorData);
        }
    }, [state.parsedValue]);

    return (
        <CodeEditorContext.Provider value={state}>
            <CodeEditorDispatchContext.Provider value={dispatch}>
                {children}
            </CodeEditorDispatchContext.Provider>
        </CodeEditorContext.Provider>
    );
};
