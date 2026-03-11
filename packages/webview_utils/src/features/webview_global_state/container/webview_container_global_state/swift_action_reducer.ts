import { EditorStateActions, WebviewContainerActions, type WebviewContainerState } from "@/code_gen/typeshare/fluster_core_utilities";
import { consola } from "consola"
import { type PayloadAction } from "@reduxjs/toolkit";
import { type AnyCrossLanguageWebviewAction, type AnyCrossLanguageEditorAction } from "#/webview_global_state/cross_language_state_types";

export const swiftContainerActionReducer = (state: WebviewContainerState, action: PayloadAction<AnyCrossLanguageWebviewAction>): WebviewContainerState => {
    consola.info("action: ", action)
    /* eslint-disable-next-line  -- This is intentionally not exhaustive, but it has a default return... */
    switch (action.payload.type) {
        case EditorStateActions.SetInitialEditorState: {
            return {
                ...state,
                ...action.payload.payload.container
            }
        }
        case WebviewContainerActions.SetDarkMode: {
            return {
                ...state,
                dark_mode: action.payload.payload.dark_mode
            }
        }
        case WebviewContainerActions.SetFlusterTheme: {
            return {
                ...state,
                fluster_theme: action.payload.payload.fluster_theme
            }
        }
        default:
            return { ...state }
    }
}
