import { type AnyCrossLanguageContainerBufferAction } from "#/split_view_editor/state/cross_language_state/cross_language_state_types";
import { type WebviewContainerState } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";
import consola from "consola";

export const swiftContainerBufferActionReducer = (state: WebviewContainerState, action: PayloadAction<AnyCrossLanguageContainerBufferAction>): WebviewContainerState => {
    consola.info("Container Action: ", action)
    return {
        ...state
    }
}
