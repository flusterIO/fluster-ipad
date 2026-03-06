import { CodeEditorBaseKeymap, CodeEditorKeymap, CodeEditorTheme, EditorSaveMethod, EditorState, EditorView } from "@/code_gen/typeshare/fluster_core_utilities";

export const initialEditorState: EditorState = {
    value: "",
    snippetProps: {
        includeEmojiSnippets: true
    },
    saveMethod: EditorSaveMethod.OnChange,
    lockEditorScrollToPreview: true,
    keymap: CodeEditorKeymap.Base,
    tags: [],
    haveSetInitialValue: false,
    theme: CodeEditorTheme.Dracula,
    editorView: EditorView.Pending,
    allCitationIds: [],
    baseKeymap: CodeEditorBaseKeymap.VsCode,
    citations: [],
    parsedValue: undefined,
    note_id: undefined,
    autoSaveTimeout: 500
}
