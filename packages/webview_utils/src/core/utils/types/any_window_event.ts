import { type ManualSaveRequestEvent, type BibtexEditorWebviewActions, type BibtexEditorWebviewEvents, type BibtexEditorWebviewLocalStorageKeys, type DictionaryWebviewActions, type DictionaryWebviewEvents, type MdxPreviewWebviewActions, type NoteDetailWebviewActions, type NoteDetailWebviewEvents, type SplitviewEditorWebviewActions, type SplitviewEditorWebviewEvents, type SplitviewEditorWebviewLocalStorageKeys, type WebviewContainerEvents, type AiStateEvents, type NoteDetailEvents } from "@/code_gen/typeshare/fluster_core_utilities";

export type AnyWebviewAction = NoteDetailWebviewActions | SplitviewEditorWebviewActions | BibtexEditorWebviewActions | DictionaryWebviewActions | MdxPreviewWebviewActions

export type AnyWebviewEvent = NoteDetailWebviewEvents | SplitviewEditorWebviewEvents | BibtexEditorWebviewEvents | DictionaryWebviewEvents | NoteDetailEvents
export type AnyWebviewStorageKey = SplitviewEditorWebviewLocalStorageKeys | BibtexEditorWebviewLocalStorageKeys

export type AnyWebviewContainerEvent = ManualSaveRequestEvent

/// --- Start new redux approach ---

export type AnyNewReduxAction = WebviewContainerEvents | AiStateEvents
