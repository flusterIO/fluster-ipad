import { type EditorStateActions, type SetEditorSaveMethodEditorAction, type SetEditorInitialStateEditorAction, type SetEditorKeymapAction, type WebviewContainerActions, type SetDarkModeAction, type SetAllCitationIdsAction, type SetAutoSaveTimeoutAction, type SetBaseKeymapAction, type SetLockEditorScrollToPreviewAction, type SetSnippetPropsAction, type SetEditorTagsAction, type SetFlusterThemeAction, type SetEditorThemeDarkAction, type SetEditorThemeLightAction, type SetParsedValueAction, type SetEditorContentAction, type SetNoteDeletedAction, type SetEditingBibEntryAction } from "@/code_gen/typeshare/fluster_core_utilities";
import { type ByteBuffer } from "flatbuffers";


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



export type AnyCrossLanguageEditorAction = EditorSaveActionRefined | EditorInitialStateActionRefined | SetEditorKeymapActionRefined | SetAllCitationIdsRefined | SetAutoSaveTimeoutActionRefined | SetBaseKeymapActionRefined | SetLockEditorScrollToPrevActionRefined | SetSnippetPropsActionRefined | SetEditorTagsActionRefined | SetEditorThemeDarkActionRefined | SetEditorThemeLightActionRefined | SetEditorContentActionRefined | SetParsedValueActionRefined | SetEditingBibEntryActionRefined;

export type AnyCrossLanguageWebviewContainerAction = SetNoteDeletedActionRefined | SetDarkModeActionRefined | SetFlusterThemeActionRefined

export type AnyCrossLanguageWebviewAction = AnyCrossLanguageEditorAction | AnyCrossLanguageWebviewContainerAction

export interface AnyCrossLanguageBufferEditorAction {
    type: EditorStateActions.SetParsedEditorContent,
    payload: ByteBuffer
}


/**
 * Any anction being send over the `handleSwiftBufferAction` bridge.
 */
export type AnyCrossLanguageContainerBufferAction = AnyCrossLanguageBufferEditorAction


export type AnyCrossLanguageEditorActionOfAnyType = AnyCrossLanguageEditorAction | AnyCrossLanguageBufferEditorAction



