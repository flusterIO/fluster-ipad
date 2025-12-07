import { CodeEditorProvider } from "#/editor/code_editor/state/code_editor_provider";
import React, { type ReactNode } from "react";
import { useMediaQuery } from "react-responsive";
import { SplitViewEditorInner } from "./split_view_editor";
import { MdxEditorPreviewOnly } from "#/mdx/components/mdx_content_preview_only";
/* import { MdxPreviewScrollPersistor } from "#/mdx/components/mdx_preview_scroll_persistor"; */

export const ResponsiveSplitViewEditor = (): ReactNode => {
    const isLandscape = useMediaQuery({
        orientation: "landscape",
    });
    return (
        <>
            <CodeEditorProvider>
                {isLandscape ? <SplitViewEditorInner /> : <MdxEditorPreviewOnly />}
            </CodeEditorProvider>
        </>
    );
};

ResponsiveSplitViewEditor.displayName = "ResponsiveSplitViewEditor";
