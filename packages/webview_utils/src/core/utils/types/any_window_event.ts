import { BibtexEditorWebviewActions, BibtexEditorWebviewEvents, NoteDetailWebviewActions, NoteDetailWebviewEvents, SplitviewEditorWebviewActions, SplitviewEditorWebviewEvents } from "@/code_gen/typeshare/fluster_core_utilities";

export type AnyWebviewAction = NoteDetailWebviewActions | SplitviewEditorWebviewActions | BibtexEditorWebviewActions
export type AnyWebviewEvent = NoteDetailWebviewEvents | SplitviewEditorWebviewEvents | BibtexEditorWebviewEvents
