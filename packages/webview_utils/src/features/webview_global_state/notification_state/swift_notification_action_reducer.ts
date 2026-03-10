import { type AnyCrossLanguageWebviewAction } from "#/split_view_editor/state/cross_language_state/cross_language_state_types";
import { type NotificationState } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";
import consola from "consola";

export const swiftNotificationActionReducer = (state: NotificationState, action: PayloadAction<AnyCrossLanguageWebviewAction>): NotificationState => {
    consola.info("Action: ", action)
    return {
        ...state
    }
}
