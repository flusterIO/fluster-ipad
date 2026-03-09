import React, { type ReactNode } from "react";
import { MdxEditorPreview, type MdxEditorPreviewProps } from "./mdx_editor_preview";
import { type EditorState, type WebviewContainerState } from "@/code_gen/typeshare/fluster_core_utilities";
import { cn } from "@/utils/cn";

/** A utility component used to implement some event listeners before rendering the MdxEditorPreview component. */
export const MdxEditorPreviewOnly = (
    { className, implementation, ...props }: Omit<MdxEditorPreviewProps, "scrollPositionKeyLandscape" | "scrollPositionKeyPortrait"> & Pick<WebviewContainerState, "implementation"> & Pick<EditorState, "parsedValue">,
): ReactNode => {
    return (
        <MdxEditorPreview
            {...props}
            className={cn("loading-hide h-full mx-auto", className, `implementation-${implementation}`)}
        />
    );
};

MdxEditorPreviewOnly.displayName = "MdxEditorPreviewOnly";
