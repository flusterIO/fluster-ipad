import { CodeEditor } from "#/editor/code_editor/components/code_editor";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
import React, { type ReactNode } from "react";
import { MdxEditorPreview } from "#/mdx/components/mdx_editor_preview";
import { CodeEditorProvider } from "#/editor/code_editor/state/code_editor_provider";

export const SplitViewEditorInner = (): ReactNode => {
    const autoSaveId = "split-view-editor-panel-split";
    return (
        <PanelGroup
            autoSaveId={autoSaveId}
            direction="horizontal"
            className="w-screen h-screen"
        >
            <Panel id="editor-panel" order={1} defaultSize={50} minSize={10}>
                <CodeEditor />
            </Panel>
            <PanelResizeHandle id="editor-panel-resize-handle" />
            <Panel
                id="editor-output-panel"
                className="bg-background"
                order={2}
                defaultSize={50}
                minSize={10}
            >
                <div
                    id="mdx-page-container"
                    className="w-full h-full overflow-y-auto overflow-x-hidden bg-background dark:bg-black"
                >
                    <MdxEditorPreview />
                </div>
            </Panel>
        </PanelGroup>
    );
};

export const SplitViewEditor = () => {
    return (
        <CodeEditorProvider>
            <SplitViewEditorInner />
        </CodeEditorProvider>
    );
};

SplitViewEditorInner.displayName = "SplitViewEditorInner";
