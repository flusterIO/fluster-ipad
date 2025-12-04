import { CodeEditor } from "#/editor/code_editor/components/code_editor";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
import React, { type ReactNode } from "react";
import { MdxEditorPreview } from "#/mdx/components/mdx_editor_preview";
import { CodeEditorProvider } from "#/editor/code_editor/state/code_editor_provider";
import {
    MDX_EDITOR_PREVIEW_ID_LANDSCAPE,
    MDX_EDITOR_PREVIEW_SCROLL_LANDSCAPE_KEY,
} from "@/state/hooks/use_persist_scroll";

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
                className="bg-white dark:bg-black"
                order={2}
                defaultSize={50}
                minSize={10}
            >
                <MdxEditorPreview
                    id={MDX_EDITOR_PREVIEW_ID_LANDSCAPE}
                    className="overflow-y-auto overflow-x-hidden h-full"
                    scrollPositionKey={MDX_EDITOR_PREVIEW_SCROLL_LANDSCAPE_KEY}
                />
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
