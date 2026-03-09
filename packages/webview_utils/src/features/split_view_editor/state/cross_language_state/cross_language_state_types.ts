import { type EditorStateActions, type SetEditorSaveMethodEditorAction, type SetEditorInitialStateEditorAction } from "@/code_gen/typeshare/fluster_core_utilities";
import { type ByteBuffer } from "flatbuffers";


interface EditorSaveActionRefined extends SetEditorSaveMethodEditorAction {
    type: EditorStateActions.SetEditorSaveMethod
}

interface EditorInitialStateActionRefined extends SetEditorInitialStateEditorAction {
    type: EditorStateActions.SetInitialEditorState,
}

export type AnyCrossLanguageEditorAction = EditorSaveActionRefined | EditorInitialStateActionRefined;

export type AnyCrossLanguageWebviewAction = AnyCrossLanguageEditorAction

export interface AnyCrossLanguageBufferEditorAction {
    type: EditorStateActions.SetParsedEditorContent,
    payload: ByteBuffer
}


/**
 * Any anction being send over the `handleSwiftBufferAction` bridge.
 */
export type AnyCrossLanguageContainerBufferAction = AnyCrossLanguageBufferEditorAction


export type AnyCrossLanguageEditorActionOfAnyType = AnyCrossLanguageEditorAction | AnyCrossLanguageBufferEditorAction



