import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import { handleSwiftAction } from '../container/webview_container_global_state/webview_container_slice'
import { type DictionaryState } from '@/code_gen/typeshare/fluster_core_utilities'
import { type AnyCrossLanguageWebviewAction } from '../cross_language_state_types'
import { initialDictionaryState } from './initial_dictionary_state'
import { swiftDictionaryActionReducer } from './dictionary_swift_action_reducer'

export interface CounterState {
    value: number
}

export const mediaSlice = createSlice({
    name: 'dictionary',
    initialState: initialDictionaryState,
    reducers: {},
    extraReducers: (builder) => {
        builder.addCase(handleSwiftAction, (state, action: PayloadAction<AnyCrossLanguageWebviewAction>): DictionaryState => {
            return swiftDictionaryActionReducer(state, action)
        })
        return builder
    }
})

// // Action creators are generated for each case reducer function
// export const { } = webviewContainerSlice.actions

export default mediaSlice.reducer
