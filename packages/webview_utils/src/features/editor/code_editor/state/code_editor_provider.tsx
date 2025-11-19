import React, { ReactNode, createContext, useReducer, useContext } from "react";
import {
    CodeEditorBaseKeymap,
    CodeEditorTheme,
} from "../types/code_editor_types";

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

    return (
        <CodeEditorContext.Provider value={state}>
            <CodeEditorDispatchContext.Provider value={dispatch}>
                {children}
            </CodeEditorDispatchContext.Provider>
        </CodeEditorContext.Provider>
    );
};
