import { type AnyCrossLanguageContainerBufferAction } from "#/webview_global_state/cross_language_state_types";
import { type WebviewContainerState } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";
import { type WithNullableOptionals } from "../../../../core/utils/types/utility_types";
import consola from "consola";

export const swiftContainerBufferActionReducer = (state: WithNullableOptionals<WebviewContainerState>, action: PayloadAction<AnyCrossLanguageContainerBufferAction>): WithNullableOptionals<WebviewContainerState> => {
    consola.info("Action: ", action)
    return {
        ...state
    }
}
