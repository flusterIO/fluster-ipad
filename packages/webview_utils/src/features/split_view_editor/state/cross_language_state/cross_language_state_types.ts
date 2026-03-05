import { OnParsedContentChangeEventBuffer } from "@/code_gen/flat_buffer/mdx-serialization";
import { EditorStateActions, SetEditorInitialStateEditorAction, SetEditorSaveMethodEditorAction } from "@/code_gen/typeshare/fluster_core_utilities";

export type AnyCrossLanguageEditorAction = {
    type: EditorStateActions.SetEditorSaveMethod
    payload: SetEditorSaveMethodEditorAction["payload"]
} | {
    type: EditorStateActions.SetInitialEditorState,
    payload: SetEditorInitialStateEditorAction["payload"]
}
export type AnyCrossLanguageBufferEditorAction = {
    type: EditorStateActions.SetParsedEditorContent,
    payload: OnParsedContentChangeEventBuffer
}
