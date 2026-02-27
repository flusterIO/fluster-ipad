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
import { useMediaQuery } from "react-responsive";
import { GetSnippetProps } from "../data/snippets/snippet_types";


type SnippetPropsState = Required<Omit<GetSnippetProps, "base" | "citationKeys">>

export enum EditorView {
    /** Set to pending initially to allow reading from media query. */
    Pending,
    Splitview,
    PreviewOnly,
}

export interface CodeEditorState {
    keymap: string;
    baseKeymap: CodeEditorBaseKeymap;
    theme: CodeEditorTheme;
    citations: CitationResultBuffer[]
    tags: TagResultBuffer[]
    allCitationIds: string[];
    value: string;
    /** pre-parsed editor content. */
    parsedValue: string | null;
    haveSetInitialValue: boolean;
    editorView: EditorView
    snippetProps: SnippetPropsState
    lockEditorScrollToPreview: boolean
}

const defaultInitialCodeEditorState: CodeEditorState = {
    baseKeymap: CodeEditorBaseKeymap.default,
    theme: CodeEditorTheme.dracula,
    keymap: "base",
    value: "",
    citations: [],
    allCitationIds: [],
    tags: [],
    parsedValue: null,
    haveSetInitialValue: false,
    editorView: EditorView.Pending,
    snippetProps: {
        // Default snippet state.
        includeEmojiSnippets: true
    },
    lockEditorScrollToPreview: false,
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
        type: "setEditorValue";
        payload: string;
    }
    | {
        type: "setParsedEditorContent";
        payload: MdxParsingResultBuffer;
    } | {
        type: "setAllCitationIds";
        payload: string[]
    } | {
        type: "setEditorView",
        payload: EditorView
    } | {
        type: "setSnippetProps",
        payload: SnippetPropsState
    } | {
        type: "setLockEditorScrollToPreview",
        payload: boolean
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
        case "setEditorValue": {
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
        case "setAllCitationIds": {
            return {
                ...state,
                allCitationIds: action.payload
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
        case "setEditorView": {
            return {
                ...state,
                editorView: action.payload
            }
        }
        case "setSnippetProps": {
            return {
                ...state,
                snippetProps: {
                    ...state.snippetProps,
                    ...action.payload
                }
            }
        }
        case "setLockEditorScrollToPreview": {
            return {
                ...state,
                lockEditorScrollToPreview: action.payload
            }
        }
    }
};

CodeEditorContextReducer.displayName = "CodeEditorContextReducer";


export type CodeEditorImplementation = "bib-editor" | "mdx-editor" | "mdx-viewer" | "development"

interface CodeEditorProviderProps {
    children: ReactNode;
    initialValues?: Partial<CodeEditorState>;
    initialValueKey?: string;
    implementation: CodeEditorImplementation
}



/**
 * Still need to add the following methods from Swift:
 * - [ ] setSnippetProps 
 *   - Still need to create action in rust, flat buffer type and all serialization from Swift.
 */
export const CodeEditorProvider = ({
    children,
    initialValues,
    initialValueKey = SplitviewEditorWebviewLocalStorageKeys.InitialValue,
    implementation
}: CodeEditorProviderProps) => {
    const [state, dispatch] = useReducer(
        CodeEditorContextReducer,
        initialValues
            ? { ...initialValues, ...defaultInitialCodeEditorState }
            : defaultInitialCodeEditorState,
    );

    const isEditorView = useMediaQuery({
        orientation: "landscape",
    });

    useEffect(() => {
        dispatch({
            type: "setEditorView",
            payload: isEditorView ? EditorView.Splitview : EditorView.PreviewOnly
        })
    }, [isEditorView])

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
                type: "setEditorValue",
                payload: initialValue,
            });
        }
    }, [initialValue, state.haveSetInitialValue]);

    /* useEventListener(SplitviewEditorWebviewEvents.SetParsedMdxContent, (e) => { */
    /*     dispatch({ */
    /*         type: "setParsedEditorContent", */
    /*         payload: e.detail, */
    /*     }); */
    /* }); */

    useEventListener(SplitviewEditorWebviewEvents.SetParsedMdxContentString, (e) => {
        dispatch({
            type: "setParsedEditorContentString",
            payload: e.detail
        })
    })

    useEventListener(SplitviewEditorWebviewEvents.SetEditorSnippetProps, (e) => {
        const ids = []
        for (let i = 0; i <= e.detail.citationIdsLength(); i++) {
            ids.push(e.detail.citationIds(i))
        }
        dispatch({
            type: "setAllCitationIds",
            payload: ids
        })
    })

    useEventListener(SplitviewEditorWebviewEvents.SetWebviewPreviewScrollLock, (e) => {
        console.log(`Received event preview-editor scroll lock event: `, e.detail)
        dispatch({
            type: "setLockEditorScrollToPreview",
            payload: e.detail
        })
    })

    useEffect(() => {
        if (state.parsedValue === null && implementation === "mdx-editor") {
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
