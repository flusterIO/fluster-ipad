import { CodeEditorProvider, EditorView, useCodeEditorContext } from "#/editor/code_editor/state/code_editor_provider";
import React, { type ReactNode } from "react";
import { SplitViewEditorInner } from "./split_view_editor";
import { MdxEditorPreviewOnly } from "#/mdx/components/mdx_content_preview_only";
import { LoadingComponent } from "@/shared_components/loading_component";
import { EditorScrollPersistor } from "#/mdx/hooks/use_persist_mdx_editor_scroll";
import { SplitviewEditorNotificationHandler } from "#/notifications/splitview_editor_notification_banner/splitview_editor_notification_banner_provider";


const EditorBody = (): ReactNode => {
    const { editorView } = useCodeEditorContext()
    if (editorView === EditorView.Pending) {
        return null
    }
    return (
        editorView === EditorView.Splitview ? <SplitViewEditorInner /> : <MdxEditorPreviewOnly implementation="mdx-editor" />
    )
}



/**
 * ResponsiveSplitViewEditor accepts children only for development so that the editor state can be modified.
 */
export const ResponsiveSplitViewEditor = ({ children = null }: { children?: ReactNode }): ReactNode => {
    return (
        <>
            <CodeEditorProvider implementation="mdx-editor">
                <SplitviewEditorNotificationHandler />
                <EditorBody />
                <div className="w-full h-full flex flex-col justify-center items-center p-8 loading-main-only hide-desktop">
                    <LoadingComponent />
                </div>
                <EditorScrollPersistor />
                {children}
            </CodeEditorProvider>
        </>
    );
};

ResponsiveSplitViewEditor.displayName = "ResponsiveSplitViewEditor";
