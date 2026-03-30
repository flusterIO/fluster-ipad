import { type EditorState, EditorStateActions, WebviewContainerActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";
import { type AnyCrossLanguageWebviewAction } from "#/webview_global_state/cross_language_state_types";
import { type WithNullableOptionals } from "../../../../core/utils/types/utility_types";

export const swiftEditorActionReducer = (state: WithNullableOptionals<EditorState>, action: PayloadAction<AnyCrossLanguageWebviewAction>): WithNullableOptionals<EditorState> => {

    /* eslint-disable-next-line  -- I know bruh... */
    switch (action.payload.type) {
        case EditorStateActions.SetInitialEditorState: {
            return {
                ...state,
                ...action.payload.payload.editor
            }
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
        case EditorStateActions.SetEditorThemeLight: {
            return {
                ...state,
                theme_light: action.payload.payload.theme_light
            }
        }
        case EditorStateActions.SetEditorThemeDark: {
            return {
                ...state,
                theme_dark: action.payload.payload.theme_dark
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
        case EditorStateActions.SetEditingBibEntry: {
            return {
                ...state,
                bib_editor: {
                    ...state.bib_editor,
                    value: action.payload.payload.content
                }
            }
        }
        case WebviewContainerActions.HandleNoteDeleted: {
            if (action.payload.payload.note_id === state.note_id) {
                return {
                    ...state,
                    note_id: null,
                    value: "",
                    parsedValue: null,
                    citations: [],
                    haveSetInitialValue: false,
                    tags: [],
                }
            } else {
                return {
                    ...state
                }
            }
        }
        default:
            return { ...state }
    }
}
