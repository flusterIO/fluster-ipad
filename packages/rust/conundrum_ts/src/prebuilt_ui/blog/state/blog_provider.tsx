"use client";
import { type ReactNode, createContext, useReducer, useContext } from "react";

export interface BlogProviderState {
    sidebar: {
        open: boolean;
    };
    query: {
        globalQuery: string;
    };
}

const defaultInitialValues: BlogProviderState = {
    sidebar: {
        open: true,
    },
    query: {
        globalQuery: "",
    },
};

export const BlogProviderContext =
    createContext<BlogProviderState>(defaultInitialValues);

type BlogProviderContextActions =
    | {
        type: "toggleSidebar";
        payload: boolean | "toggle";
    }
    | {
        type: "setNoteQuery";
        payload: string;
    };

export const BlogProviderDispatchContext = createContext<
    React.Dispatch<BlogProviderContextActions>
>(null!);

export const useBlogProviderContext = () => useContext(BlogProviderContext);
export const useBlogProviderDispatch = () =>
    useContext(BlogProviderDispatchContext);

export const BlogProviderContextReducer = (
    state: BlogProviderState,
    action: BlogProviderContextActions,
): BlogProviderState => {
    switch (action.type) {
        case "toggleSidebar": {
            return {
                ...state,
                sidebar: {
                    ...state.sidebar,
                    open:
                        action.payload === "toggle" ? !state.sidebar.open : action.payload,
                },
            };
        }
        case "setNoteQuery": {
            return {
                ...state,
                query: {
                    ...state.query,
                    globalQuery: action.payload,
                },
            };
        }
        default: {
            return state;
        }
    }
};

BlogProviderContextReducer.displayName = "BlogProviderContextReducer";

interface BlogProviderProviderProps {
    children: ReactNode;
    initialValues?: Partial<BlogProviderState>;
}

export const BlogProviderProvider = ({
    children,
    initialValues,
}: BlogProviderProviderProps) => {
    const [state, dispatch] = useReducer(
        BlogProviderContextReducer,
        initialValues
            ? { ...initialValues, ...defaultInitialValues }
            : defaultInitialValues,
    );

    return (
        <BlogProviderContext.Provider value={state}>
            <BlogProviderDispatchContext.Provider value={dispatch}>
                {children}
            </BlogProviderDispatchContext.Provider>
        </BlogProviderContext.Provider>
    );
};
