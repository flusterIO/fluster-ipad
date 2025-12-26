import React, { type ReactNode } from "react";
import { MdxEditorPreview, MdxEditorPreviewProps } from "./mdx_editor_preview";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { useCodeEditorDispatch } from "#/editor/code_editor/state/code_editor_provider";
import { SplitviewEditorWebviewEvents, SplitviewEditorWebviewIds, SplitviewEditorWebviewLocalStorageKeys } from "@/code_gen/typeshare/fluster_core_utilities";
import { cn } from "@/utils/cn";

/** A utility component used to implement some event listeners before rendering the MdxEditorPreview component. */
export const MdxEditorPreviewOnly = (
    props: MdxEditorPreviewProps,
): ReactNode => {
    const dispatch = useCodeEditorDispatch();
    useEventListener(SplitviewEditorWebviewEvents.SetSplitviewEditorContent, (e) => {
        dispatch({
            type: "setValue",
            payload: e.detail,
        });
    });
    return (
        <MdxEditorPreview
            {...props}
            className={cn("loading-main-hide h-full", props.className)}
            scrollPositionKey={SplitviewEditorWebviewLocalStorageKeys.ScrollPositionPortrait}
            id={SplitviewEditorWebviewIds.PortraitPreview}
        />
    );
};

MdxEditorPreviewOnly.displayName = "MdxEditorPreviewOnly";
