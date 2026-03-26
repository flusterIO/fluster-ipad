import { type EditorStateActions, type SetEditorSaveMethodEditorAction, type SetEditorInitialStateEditorAction, type SetEditorKeymapAction, type WebviewContainerActions, type SetDarkModeAction, type SetAllCitationIdsAction, type SetAutoSaveTimeoutAction, type SetBaseKeymapAction, type SetLockEditorScrollToPreviewAction, type SetSnippetPropsAction, type SetEditorTagsAction, type SetFlusterThemeAction, type SetEditorThemeDarkAction, type SetEditorThemeLightAction, type SetParsedValueAction, type SetEditorContentAction, type SetNoteDeletedAction, type SetEditingBibEntryAction, type NoteDetailActions, type SetNoteDetailsAction, type GlobalWebviewState, type SetDictionaryEntriesAction, type DictionaryStateActions, type SetWebviewFontSizeAction, type SetAiThinkingAction, type AiAction, type SetFoundationModelAvailabilityAction, type SetNoteSummaryAction, type SetConundrumErrorStateAction, type ConundrumStateActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { type WithNullableOptionals } from "@/utils/types/utility_types";
import { type Reducer } from "@reduxjs/toolkit";
import { type ByteBuffer } from "flatbuffers";

export type GlobalWebviewStateNullable = WithNullableOptionals<GlobalWebviewState>


export type GlobalWebviewStateDeepNullable = {
    [K in keyof GlobalWebviewStateNullable]: WithNullableOptionals<GlobalWebviewStateNullable[K]>
}

export type GlobalWebviewStateDeepNullableReducer = {
    [K in keyof GlobalWebviewStateNullable]: Reducer<WithNullableOptionals<GlobalWebviewStateNullable[K]>>
}


// -- Editor --

interface EditorSaveActionRefined extends SetEditorSaveMethodEditorAction {
    type: EditorStateActions.SetEditorSaveMethod
}

interface EditorInitialStateActionRefined extends SetEditorInitialStateEditorAction {
    type: EditorStateActions.SetInitialEditorState,
}


interface SetEditorKeymapActionRefined extends SetEditorKeymapAction {
    type: EditorStateActions.SetEditorKeymap
}


interface SetAllCitationIdsRefined extends SetAllCitationIdsAction {
    type: EditorStateActions.SetAllCitationIds
}

interface SetAutoSaveTimeoutActionRefined extends SetAutoSaveTimeoutAction {
    type: EditorStateActions.SetAutoSaveTimeout
}
interface SetBaseKeymapActionRefined extends SetBaseKeymapAction {
    type: EditorStateActions.SetBaseKeymap
}
interface SetLockEditorScrollToPrevActionRefined extends SetLockEditorScrollToPreviewAction {
    type: EditorStateActions.SetLockEditorScrollToPreview
}
interface SetSnippetPropsActionRefined extends SetSnippetPropsAction {
    type: EditorStateActions.SetSnippetProps
}
interface SetEditorTagsActionRefined extends SetEditorTagsAction {
    type: EditorStateActions.SetEditorTags
}
interface SetEditorThemeLightActionRefined extends SetEditorThemeLightAction {
    type: EditorStateActions.SetEditorThemeLight
}
interface SetEditorThemeDarkActionRefined extends SetEditorThemeDarkAction {
    type: EditorStateActions.SetEditorThemeDark
}

interface SetParsedValueActionRefined extends SetParsedValueAction {
    type: EditorStateActions.SetParsedEditorContent
}

interface SetEditorContentActionRefined extends SetEditorContentAction {
    type: EditorStateActions.SetEditorValue
}

interface SetEditingBibEntryActionRefined extends SetEditingBibEntryAction {

    type: EditorStateActions.SetEditingBibEntry
}

interface SetConundrumErrorStateActionRefined extends SetConundrumErrorStateAction {
    type: ConundrumStateActions.SetConundrumError
}


// -- Note Details --

interface SetNoteDetailsInvalidatedActionRefined {
    type: NoteDetailActions.InvalidateNoteDetails
}

export interface SetNoteDetailsActionRefined extends SetNoteDetailsAction {
    type: NoteDetailActions.SetNoteDetails,
}

export interface SetNoteSummaryActionRefined extends SetNoteSummaryAction {
    type: NoteDetailActions.SetNoteSummary
}

// -- Dictionary --

interface SetDictionaryEntriesActionRefined extends SetDictionaryEntriesAction {
    type: DictionaryStateActions.SetDictionaryEntries
}

// -- AI --

interface SetAiThinkingActionRefined extends SetAiThinkingAction {
    type: AiAction.SetAiThinking
}


interface SetFoundationModelAvailabilityActionRefined extends SetFoundationModelAvailabilityAction {
    type: AiAction.SetFoundationModelAvailability
}

// -- Container --

interface SetDarkModeActionRefined extends SetDarkModeAction {
    type: WebviewContainerActions.SetDarkMode
}

interface SetFlusterThemeActionRefined extends SetFlusterThemeAction {
    type: WebviewContainerActions.SetFlusterTheme
}

interface SetNoteDeletedActionRefined extends SetNoteDeletedAction {
    type: WebviewContainerActions.HandleNoteDeleted
}

interface SetWebviewFontSizeActionRefined extends SetWebviewFontSizeAction {
    type: WebviewContainerActions.SetWebviewFontSize
}



export type AnyCrossLanguageEditorAction = EditorSaveActionRefined | EditorInitialStateActionRefined | SetEditorKeymapActionRefined | SetAllCitationIdsRefined | SetAutoSaveTimeoutActionRefined | SetBaseKeymapActionRefined | SetLockEditorScrollToPrevActionRefined | SetSnippetPropsActionRefined | SetEditorTagsActionRefined | SetEditorThemeDarkActionRefined | SetEditorThemeLightActionRefined | SetEditorContentActionRefined | SetParsedValueActionRefined | SetEditingBibEntryActionRefined | SetConundrumErrorStateActionRefined;

export type AnyCrossLanguageNoteDetailsAction = SetNoteDetailsInvalidatedActionRefined | SetNoteDetailsActionRefined | SetNoteSummaryActionRefined

export type AnyCrossLanguageDictionaryAction = SetDictionaryEntriesActionRefined

export type AnyCrossLanguageAiAction = SetFoundationModelAvailabilityActionRefined | SetAiThinkingActionRefined

export interface AnyCrossLanguageNoteDetailsBufferAction {
    type: NoteDetailActions.SetNoteDetails
    payload: ByteBuffer
}

export type AnyCrossLanguageWebviewContainerAction = SetNoteDeletedActionRefined | SetDarkModeActionRefined | SetFlusterThemeActionRefined | SetWebviewFontSizeActionRefined

/**
 * Any cross-language action that does not have a buffer attached.
 */
export type AnyCrossLanguageWebviewAction = AnyCrossLanguageEditorAction | AnyCrossLanguageWebviewContainerAction | AnyCrossLanguageNoteDetailsAction | AnyCrossLanguageDictionaryAction | AnyCrossLanguageAiAction

export interface AnyCrossLanguageBufferEditorAction {
    type: EditorStateActions.SetParsedEditorContent,
    payload: ByteBuffer
}


/**
 * Any anction being send over the `handleSwiftBufferAction` bridge.
 */
export type AnyCrossLanguageContainerBufferAction = AnyCrossLanguageBufferEditorAction | AnyCrossLanguageNoteDetailsBufferAction



export type AnyCrossLanguageEditorActionOfAnyType = AnyCrossLanguageEditorAction | AnyCrossLanguageBufferEditorAction



