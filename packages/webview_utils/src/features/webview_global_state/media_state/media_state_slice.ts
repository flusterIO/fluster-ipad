import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import { type AnyCrossLanguageWebviewAction } from '#/split_view_editor/state/cross_language_state/cross_language_state_types'
import { handleSwiftAction } from '../container/webview_container_global_state/webview_container_slice'
import { initialMediaState } from './initial_media_state'
import { swiftMediaActionReducer } from './swift_media_action_reducer'
import { type MediaState } from '@/code_gen/typeshare/fluster_core_utilities'

export interface CounterState {
    value: number
}

export const mediaSlice = createSlice({
    name: 'media',
    initialState: initialMediaState,
    reducers: {},
    extraReducers: (builder) => {
        builder.addCase(handleSwiftAction, (state, action: PayloadAction<AnyCrossLanguageWebviewAction>): MediaState => {
            return swiftMediaActionReducer(state, action)
        })
        return builder
    }
})

// // Action creators are generated for each case reducer function
// export const { } = webviewContainerSlice.actions

export default mediaSlice.reducer
