import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import { initialEditorState } from '../initial_editor_state'
import { type AnyCrossLanguageBufferEditorAction, type AnyCrossLanguageEditorAction } from '#/split_view_editor/state/cross_language_state/cross_language_state_types'
import { type EditorState, type EditorView } from '@/code_gen/typeshare/fluster_core_utilities'
import { swiftActionReducer } from './swift_action_reducer'
import { swiftBufferActionReducer } from './swift_buffer_action_reducer'

export interface CounterState {
    value: number
}

export const editorStateSlice = createSlice({
    name: 'editor',
    initialState: initialEditorState,
    reducers: {
        setEditorView(state, action: PayloadAction<EditorView>) {
            return {
                ...state,
                editorView: action.payload
            }
        },

        /**
         * This function is attached to the window and called directly to handle all editor state interactions from Swift.
         */
        handleSwiftAction: (state, action: PayloadAction<AnyCrossLanguageEditorAction>): EditorState => {
            return {
                ...swiftActionReducer(state, action)
            }
        },

        handleSwiftBufferAction: (state, action: PayloadAction<AnyCrossLanguageBufferEditorAction>): EditorState => {
            return {
                ...swiftBufferActionReducer(state, action)
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
export const { handleSwiftAction, handleSwiftBufferAction, handleEditorChange, setEditorView } = editorStateSlice.actions

export default editorStateSlice.reducer
