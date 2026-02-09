import { CodeEditor } from "#/editor/code_editor/components/code_editor";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
import React, { type ReactNode } from "react";
import { MdxEditorPreview } from "#/mdx/components/mdx_editor_preview";
import { CodeEditorProvider } from "#/editor/code_editor/state/code_editor_provider";
import { LoadingComponent } from "@/shared_components/loading_component";
import { SplitviewEditorWebviewIds, SplitviewEditorWebviewLocalStorageKeys } from "@/code_gen/typeshare/fluster_core_utilities";

export const SplitViewEditorInner = (): ReactNode => {
    const autoSaveId = "split-view-editor-panel-split";
    return (
        <PanelGroup
            autoSaveId={autoSaveId}
            direction="horizontal"
            className="w-screen h-screen loading-main-hide"
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
                    id={SplitviewEditorWebviewIds.LandscapePreview}
                    className="overflow-y-auto overflow-x-hidden h-full loading-hide"
                    scrollPositionKeyLandscape={SplitviewEditorWebviewLocalStorageKeys.ScrollPositionLandscape}
                    scrollPositionKeyPortrait={SplitviewEditorWebviewLocalStorageKeys.ScrollPositionPortrait}
                />
                <div className="w-full h-full flex flex-col justify-center items-center loading-only-flex">
                    <LoadingComponent />
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
