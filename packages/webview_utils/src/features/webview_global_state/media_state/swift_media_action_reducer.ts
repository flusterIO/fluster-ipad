import { type AnyCrossLanguageWebviewAction } from "#/split_view_editor/state/cross_language_state/cross_language_state_types";
import { type MediaState } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";
import consola from "consola";

export const swiftMediaActionReducer = (state: MediaState, action: PayloadAction<AnyCrossLanguageWebviewAction>): MediaState => {
    consola.info("Action: ", action)
    return {
        ...state
    }
}
