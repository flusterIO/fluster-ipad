import { AiAction, type AiState } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";
import consola from "consola";
import { type AnyCrossLanguageWebviewAction } from "../cross_language_state_types";

export const swiftAiActionReducer = (state: AiState, action: PayloadAction<AnyCrossLanguageWebviewAction>): AiState => {
    consola.info("Action: ", action)
    /* eslint-disable-next-line  -- Bruh, I know it's not exhaustive... but there's a default. */
    switch (action.payload.type) {
        case AiAction.SetAiThinking: {
            return {
                ...state,
                ai_thinking: action.payload.payload.ai_thinking
            }
        }
        case AiAction.SetFoundationModelAvailability: {
            return {
                ...state,
                foundation_model_access: action.payload.payload.foundation_model_availability
            }
        }
        default: {
            return {
                ...state
            }
        }
    }
}
