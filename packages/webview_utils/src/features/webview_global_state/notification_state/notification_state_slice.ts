import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import { type EditorBannerNotification } from '@/code_gen/typeshare/fluster_core_utilities'
import { initialNotificationState } from './initial_notification_state'

export interface CounterState {
    value: number
}

export const notificationSlice = createSlice({
    name: 'notifications',
    initialState: initialNotificationState,
    reducers: {
        appendBannerNotifcation(state, action: PayloadAction<EditorBannerNotification>) {
            return {
                ...state,
                banners: [...state.banners, action.payload]
            }
        },
        removeBannerById(state, action: PayloadAction<string>) {
            return {
                ...state,
                banners: state.banners.filter((f) => f.id !== action.payload)
            }
        }
    },
    // extraReducers: (builder) => {
    //     builder.addCase(handleSwiftAction, (state, action: PayloadAction<AnyCrossLanguageWebviewAction>): NotificationState => {
    //         return swiftNotificationActionReducer(state, action)
    //     })
    //     return builder
    // }
})

// // Action creators are generated for each case reducer function
export const { appendBannerNotifcation, removeBannerById } = notificationSlice.actions

export default notificationSlice.reducer
