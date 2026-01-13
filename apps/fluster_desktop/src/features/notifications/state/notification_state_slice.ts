import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { initialNotificationsState } from "./initial_notifications_state";
import { ToastConfig } from "@fluster/desktop_bindings";
import { v4 } from "uuid";

const slice = createSlice({
    name: "notifications",
    initialState: initialNotificationsState,
    reducers: {
        appendNotification(state, action: PayloadAction<Omit<ToastConfig, "id">>) {
            const id = v4();
            state.notifications = [...state.notifications, { ...action.payload, id }];
        },
        removeNotification(state, action: PayloadAction<{ id: string }>) {
            state.notifications = state.notifications.filter(
                (f) => f.id !== action.payload.id,
            );
        },
    },
});

export const { appendNotification } = slice.actions;

export default slice.reducer;
