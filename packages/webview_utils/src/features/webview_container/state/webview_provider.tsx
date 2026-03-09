"use client"
import { isWebviewOfEnv } from "#/mdx/components/editor_dom_utils";
import { getSmallestSizableBreakpointByWidth } from "#/mdx/embeddable_mdx_components/grid/embeddable_responsive_grid_props";
import { SizableOption } from "#/mdx/embeddable_mdx_components/schemas/sizable_props_schema";
import { CodeEditorImplementation, WebviewEnvironment, type WebviewContainerState } from "@/code_gen/typeshare/fluster_core_utilities";
import React, { type ReactNode, createContext, useReducer, useContext } from "react";

const getInitialEnv = (): WebviewEnvironment | null => {
    const items = [WebviewEnvironment.IPad, WebviewEnvironment.MacOS, WebviewEnvironment.MultiPlatformDesktop]
    for (const item of items) {

        if (isWebviewOfEnv(item)) {
            return item
        }
    }
    return null
}

const initialWebviewContainerState: WebviewContainerState = {
    environment: getInitialEnv() ?? undefined,
    size: getSmallestSizableBreakpointByWidth(window.innerWidth) ?? SizableOption.None,
    dark_mode: true,
    wasm_loaded: false,
    editorImplementation: CodeEditorImplementation.Development
}

export const WebviewContainerContext = createContext<WebviewContainerState>(initialWebviewContainerState);

type WebviewContainerContextActions = { type: "set-webview-environment", payload: WebviewEnvironment } | {
    type: "set-webview-size",
    payload: SizableOption
}

/* eslint-disable-next-line  -- Been using this snippet for a while and I ain't stoppiing now. */
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
        case "set-webview-size": {
            return {
                ...state,
                size: action.payload
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
            ? { ...initialValues, ...initialWebviewContainerState }
            : initialWebviewContainerState,
    );

    return (
        <WebviewContainerContext.Provider value={state} >
            <WebviewContainerDispatchContext.Provider value={dispatch}>
                {children}
            </WebviewContainerDispatchContext.Provider>
        </WebviewContainerContext.Provider>
    )
}

