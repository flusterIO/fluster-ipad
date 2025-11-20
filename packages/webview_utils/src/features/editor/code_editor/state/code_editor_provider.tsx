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

export interface CodeEditorState {
    vimMode: boolean;
    baseKeymap: CodeEditorBaseKeymap;
    theme: CodeEditorTheme;
    value: string;
    haveSetInitialValue: boolean;
}

const defaultInitialCodeEditorState: CodeEditorState = {
    vimMode: false,
    baseKeymap: CodeEditorBaseKeymap.default,
    theme: CodeEditorTheme.dracula,
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
        type: "setVimMode";
        payload: boolean | "toggle";
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
        case "setVimMode": {
            return {
                ...state,
                vimMode: action.payload === "toggle" ? !state.vimMode : action.payload,
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
        if ((editorKeymap === "vim") !== state.vimMode) {
            dispatch({
                type: "setVimMode",
                payload: editorKeymap === "vim",
            });
        }
    }, [editorKeymap]);

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
    useEffect(() => {
        console.log(`Setting theme`);
        const payload = stringToCodeEditorTheme(
            (editorTheme as string) ?? "dracula",
        );
        if (payload !== state.theme) {
            dispatch({
                type: "setTheme",
                payload,
            });
        }
    }, [editorKeymap]);

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
        console.log(`Attempting to set initial value`);
        if (!state.haveSetInitialValue && typeof initialValue === "string") {
            console.log(`Setting initial value`);
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
