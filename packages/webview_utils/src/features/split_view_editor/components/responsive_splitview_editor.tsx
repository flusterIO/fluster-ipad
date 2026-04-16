import React, { useEffect, useRef, type ReactNode } from "react";
import { SplitViewEditorInner } from "./split_view_editor";
import { MdxEditorPreviewOnly } from "#/mdx/components/mdx_content_preview_only";
import { LoadingComponent } from "@/shared_components/loading_component";
import { EditorScrollPersistor } from "#/mdx/hooks/use_persist_mdx_editor_scroll";
import { useSelector } from "react-redux";
import { type GlobalAppState } from "#/webview_global_state/store";
import { WebviewImplementation, EditorView, SplitviewEditorWebviewActions, type EditorState } from "@/code_gen/typeshare/fluster_core_utilities";
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { connect } from 'react-redux';
import { setBibtexEditorWindowBridgeFunctions } from "#/editor/code_editor/types/swift_events/bibtex_editor_swift_events";

setBibtexEditorWindowBridgeFunctions();

const DEBOUNCE_TIMEOUT = 100
const MAX_TIME = 5000


/**
 * This is no longer _always_ rendered, so all state checks need to be lifted up.
 */
const EditorBody = (): ReactNode => {
    const editorView = useSelector((state: GlobalAppState) => state.editor.editorView)
    if (editorView === EditorView.Pending) {
        return null
    }
    return (
        editorView === EditorView.Splitview ? <SplitViewEditorInner /> : <MdxEditorPreviewOnly implementation={WebviewImplementation.MdxEditor} />
    )
}



const connectorLoadingIndicator = connect((state: GlobalAppState) => ({
    noteId: state.editor.note_id,
}))


const LoadingIndicatorAndListener = connectorLoadingIndicator(({ note_id }: Pick<EditorState, "note_id">): ReactNode => {
    const timer = useRef<null | NodeJS.Timeout>(null)
    const requestDataRecursively = (idx: number): void => {
        if (timer.current) {
            clearTimeout(timer.current)
        }
        if (idx > MAX_TIME / DEBOUNCE_TIMEOUT || note_id) {
            // TODO: Handle this error better. This should never be reached, but it needs to be handled better.
            return
        }
        timer.current = setTimeout(() => {
            sendToSwift(SplitviewEditorWebviewActions.RequestSplitviewEditorData)
            requestDataRecursively(idx + 1)
        }, DEBOUNCE_TIMEOUT)
    }

    useEffect(() => {
        requestDataRecursively(0)
        return () => {
            if (timer.current) {
                clearTimeout(timer.current)
            }
        }
    }, [])
    return (
        <div className={"w-screen h-screen fixed top-0 left-0 right-0 bottom-0 flex flex-col justify-center items-center p-4"}>
            <LoadingComponent />
        </div>
    )
})


/**
 * ResponsiveSplitViewEditor accepts children only for development so that the editor state can be modified.
 */
export const ResponsiveSplitViewEditor = ({ children = null }: { children?: ReactNode }): ReactNode => {
    const noteId = useSelector((state: GlobalAppState) => state.editor.note_id)
    return (
        <>
            {noteId ? <EditorBody /> : (
                <LoadingIndicatorAndListener />
            )}
            <EditorScrollPersistor />
            {children}
        </>
    );
};

ResponsiveSplitViewEditor.displayName = "ResponsiveSplitViewEditor";
