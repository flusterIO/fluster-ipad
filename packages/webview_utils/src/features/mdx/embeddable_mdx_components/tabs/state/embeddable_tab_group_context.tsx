"use client"
import { type ReactNode, createContext, useReducer, useContext } from "react";
import { type EmbeddableTabItem } from "../types";


export interface EmbeddableTabGroupState {
    tabs: EmbeddableTabItem[],
    focusedIndex: number,
    activeTabClasses: string,
    /// Use for making sure actions are uniquely applied to the tabgroup that they're orginated from.
    tabGroupId: string
}

const defaultInitialValues: Omit<EmbeddableTabGroupState, "activeTabClasses"> = {
    tabs: [],
    focusedIndex: 0,
    tabGroupId: ""
}

export const EmbeddableTabGroupContext = createContext<EmbeddableTabGroupState>({ ...defaultInitialValues, activeTabClasses: "" });

type EmbeddableTabGroupContextActions = { type: "addTab", payload: EmbeddableTabItem } | {
    type: "removeTab",
    /**
     * An `id` of the tab.
     */
    payload: string
} | {
    type: "setFocusedTabIndex",
    payload: number
};

/* eslint-disable-next-line  -- It won't be null for long... */
export const EmbeddableTabGroupDispatchContext = createContext<React.Dispatch<EmbeddableTabGroupContextActions>>(null!);


export const useEmbeddableTabGroupContext = () => useContext(EmbeddableTabGroupContext)
export const useEmbeddableTabGroupDispatch = () => useContext(EmbeddableTabGroupDispatchContext)


export const EmbeddableTabGroupContextReducer = (state: EmbeddableTabGroupState, action: EmbeddableTabGroupContextActions): EmbeddableTabGroupState => {
    switch (action.type) {
        case 'addTab': {
            return {
                ...state,
                tabs: state.tabs.some((x) => x.id === action.payload.id) ? state.tabs : [...state.tabs, action.payload]
            }
        }
        case "removeTab": {
            const newTabs = state.tabs.filter((f) => f.id !== action.payload)
            return {
                ...state,
                tabs: newTabs,
                focusedIndex: state.focusedIndex < newTabs.length ? state.focusedIndex : newTabs.length - 1

            }
        }
        case "setFocusedTabIndex": {
            return {
                ...state,
                focusedIndex: action.payload
            }
        }
        default: {
            return state
        }
    }
}

EmbeddableTabGroupContextReducer.displayName = "EmbeddableTabGroupContextReducer"

interface EmbeddableTabGroupProviderProps {
    children: ReactNode
    initialValues: Partial<EmbeddableTabGroupState> & { activeTabClasses: string, tabGroupId: string }
}

export const EmbeddableTabGroupProvider = ({ children, initialValues }: EmbeddableTabGroupProviderProps) => {
    const [state, dispatch] = useReducer(
        EmbeddableTabGroupContextReducer,
        { ...initialValues, ...defaultInitialValues }
    );

    return (
        <EmbeddableTabGroupContext.Provider value={state} >
            <EmbeddableTabGroupDispatchContext.Provider value={dispatch}>
                {children}
            </EmbeddableTabGroupDispatchContext.Provider>
        </EmbeddableTabGroupContext.Provider>
    )
}

