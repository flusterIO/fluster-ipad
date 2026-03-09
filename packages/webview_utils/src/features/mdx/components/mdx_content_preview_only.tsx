import React, { useEffect, type ReactNode } from "react";
import { MdxEditorPreview, type MdxEditorPreviewProps } from "./mdx_editor_preview";
import { type CodeEditorImplementation, MdxPreviewWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { cn } from "@/utils/cn";
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { useSelector } from "react-redux";
import { type MdxEditorAppState } from "#/webview_global_state/mdx_editor/store";

/** A utility component used to implement some event listeners before rendering the MdxEditorPreview component. */
export const MdxEditorPreviewOnly = (
    props: Omit<MdxEditorPreviewProps, "scrollPositionKeyLandscape" | "scrollPositionKeyPortrait"> & {
        implementation: CodeEditorImplementation
    },
): ReactNode => {
    const parsedValue = useSelector((state: MdxEditorAppState) => state.editor.parsedValue)
    useEffect(() => {
        if (typeof parsedValue === "undefined") {
            sendToSwift(MdxPreviewWebviewActions.RequestNoteData)
        }
    }, [parsedValue])
    return (
        <MdxEditorPreview
            {...props}
            className={cn("loading-hide h-full mx-auto", props.className, `implementation-${props.implementation}`)}
        />
    );
};

MdxEditorPreviewOnly.displayName = "MdxEditorPreviewOnly";
