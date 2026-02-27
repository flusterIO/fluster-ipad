import React, { useEffect, type ReactNode } from "react";
import { MdxEditorPreview, MdxEditorPreviewProps } from "./mdx_editor_preview";
import { MdxPreviewWebviewActions, SplitviewEditorWebviewIds } from "@/code_gen/typeshare/fluster_core_utilities";
import { cn } from "@/utils/cn";
import { CodeEditorImplementation, useCodeEditorContext } from "#/editor/code_editor/state/code_editor_provider";
import { sendToSwift } from "@/utils/bridge/send_to_swift";

/** A utility component used to implement some event listeners before rendering the MdxEditorPreview component. */
export const MdxEditorPreviewOnly = (
    props: Omit<MdxEditorPreviewProps, "scrollPositionKeyLandscape" | "scrollPositionKeyPortrait"> & {
        implementation: CodeEditorImplementation
    },
): ReactNode => {
    const { parsedValue } = useCodeEditorContext();
    useEffect(() => {
        if (parsedValue === null || typeof parsedValue === "undefined") {
            sendToSwift(MdxPreviewWebviewActions.RequestNoteData)
        }
    }, [parsedValue])
    return (
        <MdxEditorPreview
            {...props}
            className={cn("loading-hide h-full mx-auto", props.className, `implementation-${props.implementation}`)}
            id={SplitviewEditorWebviewIds.PortraitPreview}
        />
    );
};

MdxEditorPreviewOnly.displayName = "MdxEditorPreviewOnly";
