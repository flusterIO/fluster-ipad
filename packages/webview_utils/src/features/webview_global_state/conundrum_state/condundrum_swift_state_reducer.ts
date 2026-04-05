import { type AnyCrossLanguageWebviewAction, type GlobalWebviewStateDeepNullable } from "#/webview_global_state/cross_language_state_types";
import { ConundrumStateActions, type ConundrumState } from "@/code_gen/typeshare/fluster_core_utilities";
import { type WithNullableOptionals } from "@/utils/types/utility_types";
import { type PayloadAction } from "@reduxjs/toolkit";

export const conundrumSwiftStateReducer = (state: GlobalWebviewStateDeepNullable["conundrum"], action: PayloadAction<AnyCrossLanguageWebviewAction>): WithNullableOptionals<ConundrumState> => {
    /* eslint-disable-next-line  -- I know it's not exhaustive... */
    switch (action.payload.type) {
        case ConundrumStateActions.SetConundrumErrors: {
            return {
                ...state,
                errors: action.payload.payload
            }
        }
        default: {
            return {
                ...state
            }
        }
    }
}
