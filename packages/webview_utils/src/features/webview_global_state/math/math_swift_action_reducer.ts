import { type PayloadAction } from "@reduxjs/toolkit";
import { type AnyCrossLanguageWebviewAction } from "../cross_language_state_types";
import { EditorStateActions, type MathState } from "@/code_gen/typeshare/fluster_core_utilities";

export const mathSwiftActionReducer = (state: MathState, action: PayloadAction<AnyCrossLanguageWebviewAction>): MathState => {
    /* eslint-disable-next-line  -- I know man, I know... */
    switch (action.payload.type) {
        case EditorStateActions.SetInitialEditorState: {
            return {
                ...state,
                ...action.payload.payload.math,
                equation_refs: {}
            }
        }
        default: {
            return {
                ...state
            }
        }
    }
    }
