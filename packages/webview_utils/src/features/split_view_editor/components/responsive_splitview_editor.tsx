import { CodeEditorProvider, EditorView, useCodeEditorContext } from "#/editor/code_editor/state/code_editor_provider";
import React, { type ReactNode } from "react";
import { SplitViewEditorInner } from "./split_view_editor";
import { MdxEditorPreviewOnly } from "#/mdx/components/mdx_content_preview_only";
import { LoadingComponent } from "@/shared_components/loading_component";
import { EditorScrollPersistor } from "#/mdx/hooks/use_persist_mdx_editor_scroll";


const EditorBody = (): ReactNode => {
    const { editorView } = useCodeEditorContext()
    if (editorView === EditorView.Pending) {
        return null
    }
    return (
        editorView === EditorView.Splitview ? <SplitViewEditorInner /> : <MdxEditorPreviewOnly />
    )
}


export const ResponsiveSplitViewEditor = (): ReactNode => {
    return (
        <>
            <CodeEditorProvider implementation="mdx-editor">
                <EditorBody />
                <div className="w-full h-full flex flex-col justify-center items-center p-8 loading-main-only hide-desktop">
                    <LoadingComponent />
                </div>
                <EditorScrollPersistor />
                {/* <MdxParsingErrorIndicator /> */}
            </CodeEditorProvider>
        </>
    );
};

ResponsiveSplitViewEditor.displayName = "ResponsiveSplitViewEditor";
