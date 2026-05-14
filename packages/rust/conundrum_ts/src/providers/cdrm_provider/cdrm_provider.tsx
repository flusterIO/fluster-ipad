"use client";
import { type ReactNode, createContext, useReducer, useContext } from "react";
import { type ConundrumDocument } from "./conundrum_document";

export interface ConundrumDocsState {
    docs: ConundrumDocument[];
}

const defaultInitialValues: ConundrumDocsState = {
    docs: [],
};

export const ConundrumDocsContext =
    createContext<ConundrumDocsState>(defaultInitialValues);

type ConundrumDocsContextActions =
    | {
        type: "setConundrumDocs";
        payload: ConundrumDocument[];
    }
    | {
        type: "appendConundrumDoc";
        payload: ConundrumDocument;
    }
    | {
        type: "removeDocByPath";
        payload: string;
    };

export const ConundrumDocsDispatchContext = createContext<
    React.Dispatch<ConundrumDocsContextActions>
/* eslint-disable-next-line */
>(null!);

export const useConundrumDocsContext = () => useContext(ConundrumDocsContext);
export const useConundrumDocsDispatch = () =>
    useContext(ConundrumDocsDispatchContext);

export const ConundrumDocsContextReducer = (
    state: ConundrumDocsState,
    action: ConundrumDocsContextActions,
): ConundrumDocsState => {
    switch (action.type) {
        case "setConundrumDocs": {
            return {
                ...state,
                docs: action.payload,
            };
        }
        case "appendConundrumDoc": {
            return {
                ...state,
                docs: [...state.docs, action.payload],
            };
        }
        case "removeDocByPath": {
            return {
                ...state,
                docs: state.docs.filter((f) => f.path !== action.payload),
            };
        }
        default: {
            return state;
        }
    }
};

ConundrumDocsContextReducer.displayName = "ConundrumDocsContextReducer";

interface ConundrumDocsProviderProps {
    children: ReactNode;
    initialValues?: Partial<ConundrumDocsState>;
}

export const ConundrumDocsProvider = ({
    children,
}: ConundrumDocsProviderProps) => {
    const [state, dispatch] = useReducer(
        ConundrumDocsContextReducer,
        defaultInitialValues,
    );

    return (
        <ConundrumDocsContext.Provider value={state}>
            <ConundrumDocsDispatchContext.Provider value={dispatch}>
                {children}
            </ConundrumDocsDispatchContext.Provider>
        </ConundrumDocsContext.Provider>
    );
};
