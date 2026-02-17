import { EditorView, useCodeEditorContext } from "#/editor/code_editor/state/code_editor_provider";
import { SplitviewEditorWebviewLocalStorageKeys } from "@/code_gen/typeshare/fluster_core_utilities";
import { AnyWebviewStorageKey } from "@/utils/types/any_window_event";
import { ReactNode, useLayoutEffect } from "react"

const usePersistMdxEditorScroll = () => {

    const { editorView } = useCodeEditorContext()

    const readAndSetScroll = (ev: boolean): void => {
        const storageKey: AnyWebviewStorageKey = ev ? SplitviewEditorWebviewLocalStorageKeys.ScrollPositionSplitview : SplitviewEditorWebviewLocalStorageKeys.ScrollPositionPreviewOnly
        const value = window.localStorage.getItem(storageKey)
        if (!value) {
            return
        }
        const querySelector = {
            editor: "#scroll-target",
            previewOnly: "body"
        }[ev ? "editor" : "previewOnly"]
        const em = document.querySelector(querySelector)
        if (!em) {
            console.warn("Could not find element to persist editor scroll.")
            return
        }
        em.scrollTop = parseFloat(value)

    }

    useLayoutEffect(() => {
        if (editorView === EditorView.Pending) {
            return
        }
        readAndSetScroll(editorView === EditorView.Splitview)
    }, [editorView])
}



export const EditorScrollPersistor = (): ReactNode => {
    usePersistMdxEditorScroll()
    return null
}
