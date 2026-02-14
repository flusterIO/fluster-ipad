import React, { ComponentProps, type ReactNode } from "react";
import { setMdxPreviewWindowMethods } from "./standalone_mdx_preview_swift_events";
import { MdxEditorPreviewOnly } from "../mdx_content_preview_only";
import { CodeEditorProvider } from "#/editor/code_editor/state/code_editor_provider";

setMdxPreviewWindowMethods();

declare global {
    interface WindowEventMap {
        "set-mdx-preview-content": CustomEvent<string>;
    }
}

export const MdxStandalonePreview = (
    props: ComponentProps<typeof MdxEditorPreviewOnly>,
): ReactNode => {
    return (
        <CodeEditorProvider>
            <MdxEditorPreviewOnly {...props} />
        </CodeEditorProvider>
    )
};

MdxStandalonePreview.displayName = "MdxStandalonePreview";
