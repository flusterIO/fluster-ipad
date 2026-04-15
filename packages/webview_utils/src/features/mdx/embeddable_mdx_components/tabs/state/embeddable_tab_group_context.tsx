"use client"
import { type ReactNode, createContext, useReducer, useContext } from "react";
import { type EmbeddableTabItem } from "../types";


export interface EmbeddableTabGroupState {
    tabs: EmbeddableTabItem[],
    focusedIndex: number,
    activeTabClasses: string,
    lastIndex: number | null,
    /// Use for making sure actions are uniquely applied to the tabgroup that they're orginated from.
    tabGroupId: string
    subtle?: boolean
}

const defaultInitialValues: Omit<EmbeddableTabGroupState, "activeTabClasses" | "subtle"> = {
    tabs: [],
    focusedIndex: 0,
    tabGroupId: "",
    lastIndex: null
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
} | {
    type: "setSubtle",
    payload: boolean
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
                lastIndex: state.focusedIndex,
                focusedIndex: action.payload
            }
        }
        case "setSubtle": {
            return {
                ...state,
                subtle: action.payload
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
    initialValues: Partial<EmbeddableTabGroupState> & { activeTabClasses: string, tabGroupId: string, subtle: boolean }
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

