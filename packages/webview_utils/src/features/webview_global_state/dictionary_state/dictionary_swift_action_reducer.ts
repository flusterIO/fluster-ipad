import { DictionaryStateActions, type DictionaryState } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";
import { type AnyCrossLanguageWebviewAction } from "../cross_language_state_types";
import consola from "consola";

export const swiftDictionaryActionReducer = (state: DictionaryState, action: PayloadAction<AnyCrossLanguageWebviewAction>): DictionaryState => {
    consola.info("Action: ", action)
    /* eslint-disable-next-line  -- I know it's not exhaustive... */
    switch (action.payload.type) {
        case DictionaryStateActions.SetDictionaryEntries: {
            return {
                ...state,
                ...action.payload.payload
            }
        }
        default: {
            return {
                ...state
            }
        }
    }
}
