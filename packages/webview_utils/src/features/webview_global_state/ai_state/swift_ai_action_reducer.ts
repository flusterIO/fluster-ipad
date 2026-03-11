import { type AiState } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";
import consola from "consola";
import { type AnyCrossLanguageWebviewAction } from "../cross_language_state_types";

export const swiftAiActionReducer = (state: AiState, action: PayloadAction<AnyCrossLanguageWebviewAction>): AiState => {
    consola.info("Action: ", action)
    // switch(action.)
    return { ...state }
}
