import React, { type ComponentProps, type ReactNode } from "react";
import { MdxEditorPreviewOnly } from "../mdx_content_preview_only";

declare global {
    interface WindowEventMap {
        "set-mdx-preview-content": CustomEvent<string>;
    }
}

/**
 * @deprecated - Don't use this... it's no longer need now that we're moving 
 * to a global redux store.
 */
export const MdxStandalonePreview = (
    props: Omit<ComponentProps<typeof MdxEditorPreviewOnly>, "implementation">,
): ReactNode => {
    return (
        <MdxEditorPreviewOnly {...props} implementation="mdx-viewer" />
    )
};

MdxStandalonePreview.displayName = "MdxStandalonePreview";
