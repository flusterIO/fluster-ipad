import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import { initialWebviewContainerState } from './initial_webview_container_state'
import { type SizableOption, type WebviewContainerState } from '@/code_gen/typeshare/fluster_core_utilities'
import { swiftContainerActionReducer } from './swift_action_reducer'
import { swiftContainerBufferActionReducer } from './swift_buffer_action_reducer'
import { type AnyCrossLanguageBufferEditorAction, type AnyCrossLanguageWebviewAction } from '#/webview_global_state/cross_language_state_types'

export interface CounterState {
    value: number
}

export const webviewContainerSlice = createSlice({
    name: 'editor',
    initialState: initialWebviewContainerState as WebviewContainerState,
    reducers: {

        /**
         * This function is attached to the window and called directly to handle all editor state interactions from Swift.
         */
        handleSwiftAction: (state, action: PayloadAction<AnyCrossLanguageWebviewAction>): WebviewContainerState => {
            return swiftContainerActionReducer(state, action)
        },
        handleSwiftBufferAction: (state, action: PayloadAction<AnyCrossLanguageBufferEditorAction>): WebviewContainerState => {
            return swiftContainerBufferActionReducer(state, action)
        },
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
export const { setWasmLoaded, setDarkMode, setSize, handleSwiftAction, handleSwiftBufferAction } = webviewContainerSlice.actions

export default webviewContainerSlice.reducer
