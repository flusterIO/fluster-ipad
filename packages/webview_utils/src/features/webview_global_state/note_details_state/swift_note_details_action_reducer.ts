import { NoteDetailActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";
import { type AnyCrossLanguageWebviewAction, type GlobalWebviewStateDeepNullable } from "../cross_language_state_types";
import { type WithNullableOptionals } from "../../../core/utils/types/utility_types";
import consola from "consola";

export const swiftNoteDetailsActionReducer = (state: WithNullableOptionals<GlobalWebviewStateDeepNullable["note_details"]>, action: PayloadAction<AnyCrossLanguageWebviewAction>
): WithNullableOptionals<GlobalWebviewStateDeepNullable["note_details"]> => {
    /* eslint-disable-next-line  -- I know it's not exhaustive, but I appreciate the warning in general... */
    switch (action.payload.type) {
        case NoteDetailActions.InvalidateNoteDetails: {
            return null
        }
        case NoteDetailActions.SetNoteDetails: {
            if (action.payload.payload) {
                return {
                    /// Set everything back to null before allowing the new properties to overwrite them.
                    summary: null,
                    topic: null,
                    subject: null,
                    ...action.payload.payload
                }
            } else {
                return null
            }
        }
        case NoteDetailActions.SetNoteSummary: {
            if (state === null) {
                consola.error("Attempted to set a note summary to global state while the larger note detail state was not set.")
                return null
            } else {
                return {
                    ...state,
                    summary: {
                        ...action.payload.payload
                    }
                }
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
