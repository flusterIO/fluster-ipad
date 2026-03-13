import { NoteDetailActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";
import { type GlobalWebviewStateNullable, type AnyCrossLanguageWebviewAction } from "../cross_language_state_types";
import { type WithNullableOptionals } from "../../../core/utils/types/utility_types";

export const swiftNoteDetailsActionReducer = (state: WithNullableOptionals<GlobalWebviewStateNullable["note_details"]>, action: PayloadAction<AnyCrossLanguageWebviewAction>
): WithNullableOptionals<GlobalWebviewStateNullable["note_details"]> => {

    /* eslint-disable-next-line  -- I know it's not exhaustive, but I appreciate the warning in general... */
    switch (action.payload.type) {
        case NoteDetailActions.InvalidateNoteDetails: {
            return null
        }
        case NoteDetailActions.SetNoteDetails: {
            if (action.payload.payload) {
                return {
                    summary: null,
                    topic: null,
                    subject: null,
                    ...action.payload.payload
                }
            } else {
                return null
            }

        }
        default: {
            if (state) {
                return {
                    ...state
                }
            } else {
                return null
            }
        }
    }
}


// export const swiftNoteDetailsBufferActionReducer = (state: GlobalWebviewState["note_details"], action: PayloadAction<AnyCrossLanguageNoteDetailsBufferAction>) => {

//     switch (action.payload.type) {
//         default: {
//             return {
//                 ...state
//             }
//         }

//     }
// }
