import { type BibtexEditorState, CodeEditorBaseKeymap, CodeEditorKeymap, CodeEditorTheme, EditorSaveMethod, type EditorState, EditorView } from "@/code_gen/typeshare/fluster_core_utilities";



const initialBibEditorState: BibtexEditorState = {
    value: ""
}

export const initialEditorState: EditorState = {
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
    theme: CodeEditorTheme.Dracula,
    editorView: EditorView.Pending,
    allCitationIds: [],
    baseKeymap: CodeEditorBaseKeymap.VsCode,
    citations: [],
    parsedValue: undefined,
    note_id: undefined,
    autoSaveTimeout: 500
}
