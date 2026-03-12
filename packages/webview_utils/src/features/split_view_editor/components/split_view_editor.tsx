import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
import React, { type ReactNode } from "react";
import { MdxEditorPreview } from "#/mdx/components/mdx_editor_preview";
import { LoadingComponent } from "@/shared_components/loading_component";
import { CodeEditor } from "#/editor/code_editor/components/code_editor";
import { WebviewClient } from "#/webview_container/data/webview_client";
import { useSelector } from "react-redux";
import { type GlobalAppState } from "#/webview_global_state/store";

export const SplitViewEditorInner = (): ReactNode => {
    const autoSaveId = "split-view-editor-panel-split";
    const parsedValue = useSelector((state: GlobalAppState) => state.editor.parsedValue)
    return (
        <PanelGroup
            autoSaveId={autoSaveId}
            direction="horizontal"
            className="w-screen! h-screen!"
            onLayout={(layout) => {
                WebviewClient.sendPanelGroupResize(layout)
            }}
        >
            <Panel id="editor-panel" order={1} defaultSize={50} minSize={10}>
                <CodeEditor
                />
            </Panel>
            <PanelResizeHandle id="editor-panel-resize-handle" />
            <Panel
                id="editor-output-panel"
                className="relative"
                order={2}
                defaultSize={50}
                minSize={10}
            >
                {typeof parsedValue === "string" ? (
                    <MdxEditorPreview
                        className="overflow-y-auto overflow-x-hidden h-full min-h-screen"
                    />) : (
                    <div className="w-full h-full flex flex-col justify-center items-center">
                        <LoadingComponent />
                    </div>
                )}
            </Panel>
        </PanelGroup>
    );
};

SplitViewEditorInner.displayName = "SplitViewEditorInner";
