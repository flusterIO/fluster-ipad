import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import { type MathState } from '@/code_gen/typeshare/fluster_core_utilities'
import { type AnyCrossLanguageWebviewAction } from '../cross_language_state_types'
import { initialMathState } from './initial_math_state'
import { handleSwiftAction } from '../container/webview_container_global_state/webview_container_slice'
import { mathSwiftActionReducer } from './math_swift_action_reducer'

export interface CounterState {
    value: number
}

export const mathStateSlice = createSlice({
    name: 'math',
    initialState: initialMathState,
    reducers: {
    },
    extraReducers: (builder) => {
        builder.addCase(handleSwiftAction, (state, action: PayloadAction<AnyCrossLanguageWebviewAction>): MathState => {
            return mathSwiftActionReducer(state, action)
        })
        return builder
    }
})

// // Action creators are generated for each case reducer function
// export const { appendBannerNotifcation, removeBannerById } = notificationSlice.actions

export default mathStateSlice.reducer
