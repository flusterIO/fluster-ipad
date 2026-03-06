import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { initialEditorState } from '../initial_editor_state'
import { AnyCrossLanguageEditorActionOfAnyType } from '#/split_view_editor/state/cross_language_state/cross_language_state_types'

export interface CounterState {
    value: number
}

export const editorStateSlice = createSlice({
    name: 'editor',
    initialState: initialEditorState,
    reducers: {
        /**
         * This function is attached to the window and called directly to handle all editor state interactions from Swift.
         */
        handleSwiftAction: (state, action: PayloadAction<AnyCrossLanguageEditorActionOfAnyType>) => {
            state = {
                ...state,
                ...action.payload
            }
        },
        handleEditorChange: (state, action: PayloadAction<string>) => {
            state = {
                ...state,
                value: action.payload
            }
        }
    },
})

// Action creators are generated for each case reducer function
export const { handleSwiftAction, handleEditorChange } = editorStateSlice.actions

export default editorStateSlice.reducer
