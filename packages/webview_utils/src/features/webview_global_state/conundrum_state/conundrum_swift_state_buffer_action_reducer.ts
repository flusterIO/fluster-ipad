import { type ConundrumState, EditorStateActions, SplitviewEditorWebviewEvents } from "@/code_gen/typeshare/fluster_core_utilities";
import { type WithNullableOptionals } from "@/utils/types/utility_types";
import { type PayloadAction } from "@reduxjs/toolkit";
import { type AnyCrossLanguageBufferEditorAction } from "../cross_language_state_types";

export const conundrumSwiftBufferActionReducer = (state: WithNullableOptionals<ConundrumState>, action: PayloadAction<AnyCrossLanguageBufferEditorAction>) => {
    switch (action.payload.type) {
        /* eslint-disable-next-line  -- I knowit's always true I'm future proofing, because I'm not writing this shit twice... */
        case EditorStateActions.SetParsedEditorContent: {
            window.dispatchEvent(new CustomEvent(SplitviewEditorWebviewEvents.ErrorStateReset, {
                detail: null
            }));
            return {
                ...state,
                errors: []
            }
        }
        default: {
            return {
                ...state
            }
        }
    }
}
