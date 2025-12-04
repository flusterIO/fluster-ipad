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
import { sendToSwift, SwiftHandler } from "@/utils/bridge/send_to_swift";

export interface CodeEditorState {
    keymap: string;
    baseKeymap: CodeEditorBaseKeymap;
    theme: CodeEditorTheme;
    value: string;
    parsedValue: string | null;
    haveSetInitialValue: boolean;
}

const defaultInitialCodeEditorState: CodeEditorState = {
    baseKeymap: CodeEditorBaseKeymap.default,
    theme: CodeEditorTheme.dracula,
    keymap: "base",
    value: "",
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
        type: "setInitialEditorValue";
        payload: string;
    }
    | {
        type: "setParsedEditorContent";
        payload: string;
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
        case "setParsedEditorContent": {
            return {
                ...state,
                parsedValue: action.payload,
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
    initialValueKey = "editor-initial-value",
}: CodeEditorProviderProps) => {
    const [state, dispatch] = useReducer(
        CodeEditorContextReducer,
        initialValues
            ? { ...initialValues, ...defaultInitialCodeEditorState }
            : defaultInitialCodeEditorState,
    );

    const [editorKeymap] = useLocalStorage("editor-keymap", "base", {
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

    useEventListener("set-editor-keymap", (e) => {
        dispatch({
            type: "setKeymap",
            payload: e.detail,
        });
    });

    const [editorTheme] = useLocalStorage("editor-theme", undefined, {
        deserializer(value) {
            return value;
        },
        serializer(value) {
            return value;
        },
        initializeWithValue: false,
    });
    const handleTheme = (t: string): void => {
        console.log("t: ", t);
        const payload = stringToCodeEditorTheme((t as string) ?? "dracula");
        dispatch({
            type: "setTheme",
            payload,
        });
    };
    useEffect(() => {
        handleTheme(editorTheme);
    }, [editorTheme]);
    useEventListener("set-code-theme", (e) => {
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

    useEventListener("set-parsed-editor-content", (e) => {
        dispatch({
            type: "setParsedEditorContent",
            payload: e.detail,
        });
    });

    useEffect(() => {
        console.log("state.parsedValue: ", state.parsedValue);
        if (state.parsedValue === null) {
            sendToSwift(SwiftHandler.requestParsedMdxContent, "");
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
