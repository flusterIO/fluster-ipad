import { type AnyCrossLanguageEditorAction } from "#/split_view_editor/state/cross_language_state/cross_language_state_types";
import { EditorStateActions, type WebviewContainerState } from "@/code_gen/typeshare/fluster_core_utilities";
import { consola } from "consola"
import { type PayloadAction } from "@reduxjs/toolkit";

export const swiftContainerActionReducer = (state: WebviewContainerState, action: PayloadAction<AnyCrossLanguageEditorAction>): WebviewContainerState => {
    consola.info("action: ", action)
    /* eslint-disable-next-line  -- This is intentionally not exhaustive, but it has a default return... */
    switch (action.payload.type) {
        case EditorStateActions.SetInitialEditorState: {
            return {
                ...state,
                ...action.payload.payload.container
            }
        }
        default:
            return { ...state }
    }
    }
