import { CodeEditorProvider } from "#/editor/code_editor/state/code_editor_provider";
import React, { type ReactNode } from "react";
import { useMediaQuery } from "react-responsive";
import { SplitViewEditorInner } from "./split_view_editor";
import { MdxEditorPreview } from "#/mdx/components/mdx_editor_preview";

export const ResponsiveSplitViewEditor = (): ReactNode => {
    const isLanscape = useMediaQuery({
        orientation: "landscape",
    });
    return (
        <CodeEditorProvider>
            {isLanscape ? <SplitViewEditorInner /> : <MdxEditorPreview />}
        </CodeEditorProvider>
    );
};

ResponsiveSplitViewEditor.displayName = "ResponsiveSplitViewEditor";
