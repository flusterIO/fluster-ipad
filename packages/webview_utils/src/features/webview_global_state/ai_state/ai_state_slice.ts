import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import { initialAiState } from './initial_ai_state'
import { swiftAiActionReducer } from './swift_ai_action_reducer'
import { type AnyCrossLanguageWebviewAction } from '#/split_view_editor/state/cross_language_state/cross_language_state_types'
import { handleSwiftAction } from '../container/webview_container_global_state/webview_container_slice'

export interface CounterState {
    value: number
}

export const aiSlice = createSlice({
    name: 'ai',
    initialState: initialAiState,
    reducers: {},
    extraReducers: (builder) => {
        builder.addCase(handleSwiftAction, (state, action: PayloadAction<AnyCrossLanguageWebviewAction>) => {
            return swiftAiActionReducer(state, action)
        })
        return builder
    }
})

// // Action creators are generated for each case reducer function
// export const { } = webviewContainerSlice.actions

export default aiSlice.reducer
