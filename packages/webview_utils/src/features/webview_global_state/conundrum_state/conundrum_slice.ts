import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import { type ConundrumState } from '@/code_gen/typeshare/fluster_core_utilities'
import { initialConundrumState } from './initial_conundrum_state'
import { conundrumSwiftStateReducer } from './condundrum_swift_state_reducer'
import { type WithNullableOptionals } from '@/utils/types/utility_types'
import { handleSwiftAction } from '#/webview_global_state/container/webview_container_global_state/webview_container_slice'
import { type AnyCrossLanguageWebviewAction } from '#/webview_global_state/cross_language_state_types'

export interface CounterState {
    value: number
}

export const notificationSlice = createSlice({
    name: 'notifications',
    initialState: initialConundrumState,
    reducers: {
        clearConundrumError(state) {
            return {
                ...state,
                error: null
            }
        }
    },
    extraReducers: (builder) => {
        builder.addCase(handleSwiftAction, (state, action: PayloadAction<AnyCrossLanguageWebviewAction>): WithNullableOptionals<ConundrumState> => {
            return conundrumSwiftStateReducer(state, action)
        })
        return builder
    }
})

// // Action creators are generated for each case reducer function
export const { clearConundrumError } = notificationSlice.actions

export default notificationSlice.reducer
