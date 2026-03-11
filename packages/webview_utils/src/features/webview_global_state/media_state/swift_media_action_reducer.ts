import { type MediaState } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";
import consola from "consola";
import { type AnyCrossLanguageWebviewAction } from "../cross_language_state_types";

export const swiftMediaActionReducer = (state: MediaState, action: PayloadAction<AnyCrossLanguageWebviewAction>): MediaState => {
    consola.info("Action: ", action)
    return {
        ...state
    }
}
