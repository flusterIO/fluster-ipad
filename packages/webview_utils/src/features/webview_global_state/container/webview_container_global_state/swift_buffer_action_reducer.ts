import { type AnyCrossLanguageContainerBufferAction } from "#/webview_global_state/cross_language_state_types";
import { type WebviewContainerState } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";
import consola from "consola";
import { type WithNullableOptionals } from "../../../../core/utils/types/utility_types";

export const swiftContainerBufferActionReducer = (state: WithNullableOptionals<WebviewContainerState>, action: PayloadAction<AnyCrossLanguageContainerBufferAction>): WithNullableOptionals<WebviewContainerState> => {
    consola.info("Container Action: ", action)
    return {
        ...state
    }
}
