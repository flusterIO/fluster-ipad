import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { initialNotificationState } from "./initial_notification_state";
import { type NotificationItem } from "../models/notifcation_item";

const searchSlice = createSlice({
    name: "notification",
    initialState: initialNotificationState,
    reducers: {
        appendNotification(state, action: PayloadAction<NotificationItem>) {
            state.notifications = [...state.notifications, action.payload];
        },

        removeNotificationById(state, action: PayloadAction<string>) {
            state.notifications = state.notifications.filter(
                (f) => f.id !== action.payload,
            );
        },
    },
});

export const { appendNotification, removeNotificationById } =
    searchSlice.actions;

export default searchSlice.reducer;
