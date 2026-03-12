import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import { initialEditorState } from '../initial_editor_state'
import { type EditorView } from '@/code_gen/typeshare/fluster_core_utilities'
import { swiftEditorActionReducer } from './swift_action_reducer'
import { handleSwiftAction, handleSwiftBufferAction } from '#/webview_global_state/container/webview_container_global_state/webview_container_slice'
import { swiftEditorBufferActionReducer } from './swift_buffer_action_reducer'
import { type AnyCrossLanguageWebviewAction } from '#/webview_global_state/cross_language_state_types'

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
        setEditorValue(state, action: PayloadAction<string>) {
            return {
                ...state,
                value: action.payload
            }
        },
        setBibtexEditorValue(state, action: PayloadAction<string>) {
            return {
                ...state,
                bib_editor: {
                    ...state.bib_editor,
                    value: action.payload
                }
            }
        }
    },
    extraReducers: (builder) => {
        builder.addCase(handleSwiftAction, (state, action: PayloadAction<AnyCrossLanguageWebviewAction>) => {
            return swiftEditorActionReducer(state, action)
        })
        builder.addCase(handleSwiftBufferAction, (state, action) => {
            return swiftEditorBufferActionReducer(state, action)
        })
        return builder
    }
})

// Action creators are generated for each case reducer function
export const { setEditorValue, setEditorView, setBibtexEditorValue } = editorStateSlice.actions

export default editorStateSlice.reducer
