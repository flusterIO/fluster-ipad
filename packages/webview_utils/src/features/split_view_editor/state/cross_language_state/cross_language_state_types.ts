import { type EditorStateActions, type SetEditorSaveMethodEditorAction, type SetEditorInitialStateEditorAction, type SetEditorKeymapAction, type WebviewContainerActions, type SetDarkModeAction, type SetAllCitationIdsAction, SetAutoSaveTimeoutPayload, type SetAutoSaveTimeoutAction, type SetBaseKeymapAction, type SetLockEditorScrollToPreviewAction, type SetSnippetPropsAction, type SetEditorTagsAction, type SetFlusterThemeAction, type SetEditorThemeAction, type SetParsedValueAction, type SetEditorContentAction } from "@/code_gen/typeshare/fluster_core_utilities";
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
interface SetEditorThemeActionRefined extends SetEditorThemeAction {
    type: EditorStateActions.SetEditorTheme
}

interface SetParsedValueActionRefined extends SetParsedValueAction {
    type: EditorStateActions.SetParsedEditorContent
}

interface SetEditorContentActionRefined extends SetEditorContentAction {
    type: EditorStateActions.SetEditorValue
}


// -- Container --

interface SetDarkModeActionRefined extends SetDarkModeAction {
    type: WebviewContainerActions.SetDarkMode
}

interface SetFlusterThemeActionRefined extends SetFlusterThemeAction {
    type: WebviewContainerActions.SetFlusterTheme
}





export type AnyCrossLanguageEditorAction = EditorSaveActionRefined | EditorInitialStateActionRefined | SetEditorKeymapActionRefined | SetDarkModeActionRefined | SetAllCitationIdsRefined | SetAutoSaveTimeoutActionRefined | SetBaseKeymapActionRefined | SetLockEditorScrollToPrevActionRefined | SetSnippetPropsActionRefined | SetEditorTagsActionRefined | SetFlusterThemeActionRefined | SetEditorThemeActionRefined | SetEditorContentActionRefined | SetParsedValueActionRefined;

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



