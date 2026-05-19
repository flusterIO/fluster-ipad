"use client";
import { type ReactNode, createContext, useReducer, useContext } from "react";
import { type BlogSearchParams } from "../types";

export interface BlogProviderState {
    sidebar: {
        open: boolean;
    };
    query: {
        globalQuery: string;
        searchParams?: BlogSearchParams;
    };
}

const defaultInitialValues: BlogProviderState = {
    sidebar: {
        open: true,
    },
    query: {
        globalQuery: "",
        searchParams: {
            slug: [],
        },
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
    searchParams?: BlogSearchParams;
}

export const BlogProviderProvider = ({
    children,
    searchParams,
}: BlogProviderProviderProps) => {
    const [state, dispatch] = useReducer(BlogProviderContextReducer, {
        ...defaultInitialValues,
        query: {
            ...defaultInitialValues.query,
            searchParams,
        },
    });

    return (
        <BlogProviderContext.Provider value={state}>
            <BlogProviderDispatchContext.Provider value={dispatch}>
                {children}
            </BlogProviderDispatchContext.Provider>
        </BlogProviderContext.Provider>
    );
};
