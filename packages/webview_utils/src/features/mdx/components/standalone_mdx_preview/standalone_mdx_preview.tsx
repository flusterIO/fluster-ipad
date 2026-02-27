import React, { ComponentProps, type ReactNode } from "react";
import { MdxEditorPreviewOnly } from "../mdx_content_preview_only";
import { CodeEditorProvider } from "#/editor/code_editor/state/code_editor_provider";

declare global {
    interface WindowEventMap {
        "set-mdx-preview-content": CustomEvent<string>;
    }
}

export const MdxStandalonePreview = (
    props: Omit<ComponentProps<typeof MdxEditorPreviewOnly>, "implementation">,
): ReactNode => {
    return (
        <CodeEditorProvider implementation="mdx-viewer">
            <MdxEditorPreviewOnly {...props} implementation="mdx-viewer" />
        </CodeEditorProvider>
    )
};

MdxStandalonePreview.displayName = "MdxStandalonePreview";
