import React, { type ReactNode } from "react";
import { SplitViewEditorInner } from "./split_view_editor";
import { MdxEditorPreviewOnly } from "#/mdx/components/mdx_content_preview_only";
import { LoadingComponent } from "@/shared_components/loading_component";
import { EditorScrollPersistor } from "#/mdx/hooks/use_persist_mdx_editor_scroll";
import { SplitviewEditorNotificationHandler } from "#/notifications/splitview_editor_notification_banner/splitview_editor_notification_banner_provider";
import { useSelector } from "react-redux";
import { type MdxEditorAppState } from "#/webview_global_state/mdx_editor/store";
import { CodeEditorImplementation, EditorView } from "@/code_gen/typeshare/fluster_core_utilities";
import { cn } from "@/utils/cn";


/**
 * This is no longer _always_ rendered, so all state checks need to be lifted up.
 */
const EditorBody = (): ReactNode => {
    const editorView = useSelector((state: MdxEditorAppState) => state.editor.editorView)
    if (editorView === EditorView.Pending) {
        return null
    }
    return (
        editorView === EditorView.Splitview ? <SplitViewEditorInner /> : <MdxEditorPreviewOnly implementation={CodeEditorImplementation.MdxEditor} />
    )
}


/**
 * ResponsiveSplitViewEditor accepts children only for development so that the editor state can be modified.
 */
export const ResponsiveSplitViewEditor = ({ children = null }: { children?: ReactNode }): ReactNode => {
    const noteId = useSelector((state: MdxEditorAppState) => state.editor.note_id)
    return (
        <>
            <SplitviewEditorNotificationHandler />
            {noteId ? <EditorBody /> : (
                <div className={cn("w-screen h-screen fixed top-0 left-0 right-0 bottom-0 flex flex-col justify-center items-center p-4", noteId && "hidden")}>
                    <LoadingComponent />
                </div>
            )}
            <EditorScrollPersistor />
            {children}
        </>
    );
};

ResponsiveSplitViewEditor.displayName = "ResponsiveSplitViewEditor";
