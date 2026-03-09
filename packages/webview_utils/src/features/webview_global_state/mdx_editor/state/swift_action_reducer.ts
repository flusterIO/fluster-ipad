import { type AnyCrossLanguageEditorAction } from "#/split_view_editor/state/cross_language_state/cross_language_state_types";
import { type EditorState, EditorStateActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { consola } from "consola"
import { type PayloadAction } from "@reduxjs/toolkit";

export const swiftActionReducer = (state: EditorState, action: PayloadAction<AnyCrossLanguageEditorAction>): EditorState => {
    consola.info("action: ", action)
    switch (action.payload.type) {
        case EditorStateActions.SetInitialEditorState: {
            return {
                ...state,
                ...action.payload.payload
            }
        }
        case EditorStateActions.SetEditorSaveMethod: {
            return {
                ...state,
                saveMethod: action.payload.payload
            }
        }
    }
    }
