import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import { type ConundrumState } from '@/code_gen/typeshare/fluster_core_utilities'
import { initialConundrumState } from './initial_conundrum_state'
import { conundrumSwiftStateReducer } from './condundrum_swift_state_reducer'
import { type WithNullableOptionals } from '@/utils/types/utility_types'
import { handleSwiftAction, handleSwiftBufferAction } from '#/webview_global_state/container/webview_container_global_state/webview_container_slice'
import { type AnyCrossLanguageBufferEditorAction, type AnyCrossLanguageWebviewAction } from '#/webview_global_state/cross_language_state_types'
import { type ConundrumError } from '@/code_gen/typeshare/conundrum'
import { conundrumSwiftBufferActionReducer } from './conundrum_swift_state_buffer_action_reducer'

export const conundrumSlice = createSlice({
    name: 'conundrum',
    initialState: initialConundrumState,
    reducers: {
        setConundrumErrors(state, action: PayloadAction<ConundrumError[]>) {
            return {
                ...state,
                errors: action.payload
            }
        },
        appendConundrumError(state, action: PayloadAction<ConundrumError>) {
            return {
                ...state,
                errors: [...state.errors, action.payload]
            }
        },
        clearConundrumErrors(state) {
            return {
                ...state,
                errors: []
            }
        },
    },
    extraReducers: (builder) => {
        builder.addCase(handleSwiftAction, (state, action: PayloadAction<AnyCrossLanguageWebviewAction>): WithNullableOptionals<ConundrumState> => {
            return conundrumSwiftStateReducer(state, action)
        })
        builder.addCase(handleSwiftBufferAction, (state, action: PayloadAction<AnyCrossLanguageBufferEditorAction>): WithNullableOptionals<ConundrumState> => {
            return conundrumSwiftBufferActionReducer(state, action)
        })
        return builder
    }
})

// // Action creators are generated for each case reducer function
export const { clearConundrumErrors, appendConundrumError, setConundrumErrors } = conundrumSlice.actions

export default conundrumSlice.reducer
