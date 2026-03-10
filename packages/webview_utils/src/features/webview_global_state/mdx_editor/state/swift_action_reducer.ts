import { type AnyCrossLanguageEditorAction } from "#/split_view_editor/state/cross_language_state/cross_language_state_types";
import { type EditorState, EditorStateActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { consola } from "consola"
import { type PayloadAction } from "@reduxjs/toolkit";

export const swiftEditorActionReducer = (state: EditorState, action: PayloadAction<AnyCrossLanguageEditorAction>): EditorState => {
    consola.info("action: ", action)
    /* eslint-disable-next-line  -- I know it's non-exhaustive, but I appreciate it... */
    switch (action.payload.type) {
        case EditorStateActions.SetInitialEditorState: {
            return {
                ...state,
                ...action.payload.payload.editor
            } satisfies EditorState
        }
        case EditorStateActions.SetEditorSaveMethod: {
            return {
                ...state,
                saveMethod: action.payload.payload
            }
        }
        case EditorStateActions.SetAllCitationIds: {
            return {
                ...state,
                allCitationIds: action.payload.payload.all_citation_ids
            }
        }
        case EditorStateActions.SetAutoSaveTimeout: {
            return {
                ...state,
                autoSaveTimeout: action.payload.payload.auto_save_timeout
            }
        }
        case EditorStateActions.SetBaseKeymap: {
            return {
                ...state,
                baseKeymap: action.payload.payload.base_keymap
            }
        }
        case EditorStateActions.SetEditorKeymap: {
            return {
                ...state,
                keymap: action.payload.payload.keymap
            }
        }
        case EditorStateActions.SetEditorTags: {
            return {
                ...state,
                tags: action.payload.payload.tags
            }
        }
        case EditorStateActions.SetEditorTheme: {
            return {
                ...state,
                theme: action.payload.payload.theme
            }
        }
        case EditorStateActions.SetLockEditorScrollToPreview: {
            return {
                ...state,
                lockEditorScrollToPreview: action.payload.payload.lock_editor_scroll_to_preview
            }
        }
        case EditorStateActions.SetSnippetProps: {
            return {
                ...state,
                snippetProps: action.payload.payload.snippetProps
            }
        }
        case EditorStateActions.SetEditorValue: {
            return {
                ...state,
                value: action.payload.payload.value
            }
        }
        case EditorStateActions.SetParsedEditorContent: {
            return {
                ...state,
                parsedValue: action.payload.payload.value
            }
        }
        default:
            return { ...state }
    }
}
