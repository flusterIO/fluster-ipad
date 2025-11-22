import { CodeEditorProvider } from "#/editor/code_editor/state/code_editor_provider";
import React, { type ReactNode } from "react";
import { useMediaQuery } from "react-responsive";
import { SplitViewEditorInner } from "./split_view_editor";
import { MdxEditorPreviewOnly } from "#/mdx/components/mdx_content_preview_only";

export const ResponsiveSplitViewEditor = (): ReactNode => {
    const isLanscape = useMediaQuery({
        orientation: "landscape",
    });
    return (
        <CodeEditorProvider>
            {isLanscape ? <SplitViewEditorInner /> : <MdxEditorPreviewOnly />}
        </CodeEditorProvider>
    );
};

ResponsiveSplitViewEditor.displayName = "ResponsiveSplitViewEditor";
