"use client"
import { isWebviewOfEnv } from "#/mdx/components/editor_dom_utils";
import { WebviewEnvironment } from "@/code_gen/typeshare/fluster_core_utilities";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { ReactNode, createContext, useReducer, useContext } from "react";

export interface WebviewContainerState {
    environment: WebviewEnvironment | null
}

const getInitialEnv = (): WebviewEnvironment | null => {
    if (!document.body) {
        return null
    }
    const items = [WebviewEnvironment.IPad, WebviewEnvironment.MacOS, WebviewEnvironment.MultiPlatformDesktop]
    for (const item of items) {

        if (isWebviewOfEnv(item)) {
            return item
        }
    }
    return null
}

const defaultInitialValues: WebviewContainerState = {
    environment: getInitialEnv()
}

export const WebviewContainerContext = createContext<WebviewContainerState>(defaultInitialValues);

type WebviewContainerContextActions = { type: "set-webview-environment", payload: WebviewEnvironment }

export const WebviewContainerDispatchContext = createContext<React.Dispatch<WebviewContainerContextActions>>(null!);


export const useWebviewContainerContext = () => useContext(WebviewContainerContext)
export const useWebviewContainerDispatch = () => useContext(WebviewContainerDispatchContext)


export const WebviewContainerContextReducer = (state: WebviewContainerState, action: WebviewContainerContextActions): WebviewContainerState => {
    switch (action.type) {
        case 'set-webview-environment': {
            return {
                ...state,
                environment: action.payload
            }
        }
        default: {
            return state
        }
    }
}

WebviewContainerContextReducer.displayName = "WebviewContainerContextReducer"

interface WebviewContainerProviderProps {
    children: ReactNode
    initialValues?: Partial<WebviewContainerState>
}

export const WebviewContainerProvider = ({ children, initialValues }: WebviewContainerProviderProps) => {
    const [state, dispatch] = useReducer(
        WebviewContainerContextReducer,
        initialValues
            ? { ...initialValues, ...defaultInitialValues }
            : defaultInitialValues,
    );

    return (
        <WebviewContainerContext.Provider value={state} >
            <WebviewContainerDispatchContext.Provider value={dispatch}>
                {children}
            </WebviewContainerDispatchContext.Provider>
        </WebviewContainerContext.Provider>
    )
}

