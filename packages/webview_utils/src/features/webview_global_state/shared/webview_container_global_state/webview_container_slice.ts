import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { initialWebviewContainerState } from './initial_webview_container_state'
import { SizableOption, WebviewContainerState } from '@/code_gen/typeshare/fluster_core_utilities'

export interface CounterState {
    value: number
}

export const webviewContainerSlice = createSlice({
    name: 'editor',
    initialState: initialWebviewContainerState as WebviewContainerState,
    reducers: {
        setWasmLoaded(state) {
            return {
                ...state,
                wasm_loaded: true
            }
        },
        setSize(state, action: PayloadAction<SizableOption>) {
            return {
                ...state,
                size: action.payload
            }
        },
        setDarkMode(state, action: PayloadAction<boolean | "toggle">) {
            return {
                ...state,
                dark_mode: action.payload === "toggle" ? !state.dark_mode : action.payload
            }
        }
    },
})

// Action creators are generated for each case reducer function
export const { setWasmLoaded, setDarkMode, setSize } = webviewContainerSlice.actions

export default webviewContainerSlice.reducer
