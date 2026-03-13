import { type BibtexEditorState, CodeEditorBaseKeymap, CodeEditorKeymap, CodeEditorTheme, EditorSaveMethod, type EditorState, EditorView } from "@/code_gen/typeshare/fluster_core_utilities";
import { type WithNullableOptionals } from "../../../core/utils/types/utility_types";



const initialBibEditorState: BibtexEditorState = {
    value: "",
}

export const initialEditorState: WithNullableOptionals<EditorState> = {
    value: "",
    snippetProps: {
        includeEmojiSnippets: true
    },
    bib_editor: initialBibEditorState,
    saveMethod: EditorSaveMethod.OnChange,
    lockEditorScrollToPreview: true,
    keymap: CodeEditorKeymap.Base,
    tags: [],
    haveSetInitialValue: false,
    theme_dark: CodeEditorTheme.Dracula,
    theme_light: CodeEditorTheme.MaterialLight,
    editorView: EditorView.Pending,
    allCitationIds: [],
    baseKeymap: CodeEditorBaseKeymap.VsCode,
    citations: [],
    parsedValue: null,
    note_id: null,
    autoSaveTimeout: 500
}
