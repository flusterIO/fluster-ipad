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

export interface CodeEditorState {
    keymap: string;
    baseKeymap: CodeEditorBaseKeymap;
    theme: CodeEditorTheme;
    value: string;
    haveSetInitialValue: boolean;
}

const defaultInitialCodeEditorState: CodeEditorState = {
    baseKeymap: CodeEditorBaseKeymap.default,
    theme: CodeEditorTheme.dracula,
    keymap: "base",
    value: "",
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
        default: {
            return state;
        }
    }
};

CodeEditorContextReducer.displayName = "CodeEditorContextReducer";

interface CodeEditorProviderProps {
    children: ReactNode;
    initialValues?: Partial<CodeEditorState>;
}

export const CodeEditorProvider = ({
    children,
    initialValues,
}: CodeEditorProviderProps) => {
    const [state, dispatch] = useReducer(
        CodeEditorContextReducer,
        initialValues
            ? { ...initialValues, ...defaultInitialCodeEditorState }
            : defaultInitialCodeEditorState,
    );

    const [editorKeymap] = useLocalStorage("editor-keymap", undefined, {
        deserializer(value) {
            console.log("value: ", value);
            return value;
        },
        serializer(value) {
            console.log("value: ", value);
            return value;
        },
        initializeWithValue: false,
    });
    useEffect(() => {
        console.log(`Setting editor keymap`);
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
            console.log("value: ", value);
            return value;
        },
        serializer(value) {
            console.log("value: ", value);
            return value;
        },
        initializeWithValue: false,
    });
    const handleTheme = (t: string): void => {
        console.log(`Setting theme`);
        const payload = stringToCodeEditorTheme((t as string) ?? "dracula");
        dispatch({
            type: "setTheme",
            payload,
        });
    };
    useEffect(() => {
        handleTheme(editorTheme);
    }, [editorTheme]);
    useEventListener("set-editor-theme", (e) => {
        handleTheme(e.detail);
    });

    const [initialValue] = useLocalStorage("editor-initial-value", undefined, {
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

    return (
        <CodeEditorContext.Provider value={state}>
            <CodeEditorDispatchContext.Provider value={dispatch}>
                {children}
            </CodeEditorDispatchContext.Provider>
        </CodeEditorContext.Provider>
    );
};
