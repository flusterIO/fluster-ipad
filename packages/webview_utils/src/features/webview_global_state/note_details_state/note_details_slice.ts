import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import { handleSwiftAction } from '../container/webview_container_global_state/webview_container_slice'
import { type GlobalWebviewStateNullable, type AnyCrossLanguageWebviewAction } from '../cross_language_state_types'
import { swiftNoteDetailsActionReducer } from './swift_note_details_action_reducer'

export interface CounterState {
    value: number
}

export const noteDetailsSlice = createSlice({
    name: 'note_details',
    initialState: null as GlobalWebviewStateNullable["note_details"],
    reducers: {},
    extraReducers: (builder) => {
        builder.addCase(handleSwiftAction, (state, action: PayloadAction<AnyCrossLanguageWebviewAction>): GlobalWebviewStateNullable["note_details"] => {

            return swiftNoteDetailsActionReducer(state, action);
        })
        return builder
    }
})

// // Action creators are generated for each case reducer function
// export const { } = webviewContainerSlice.actions

export default noteDetailsSlice.reducer
