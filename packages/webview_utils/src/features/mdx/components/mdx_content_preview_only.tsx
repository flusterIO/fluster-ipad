import React, { type ReactNode } from "react";
import { MdxEditorPreview, MdxEditorPreviewProps } from "./mdx_editor_preview";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { useCodeEditorDispatch } from "#/editor/code_editor/state/code_editor_provider";
import { SplitviewEditorWebviewEvents } from "@/code_gen/typeshare/fluster_core_utilities";
import { MDX_EDITOR_PREVIEW_ID_PORTRAIT, MDX_EDITOR_PREVIEW_SCROLL_PORTRAIT_KEY } from "../data/mdx_scroll_restore_keys";
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
            className={cn("loading-main-hide", props.className)}
            scrollPositionKey={MDX_EDITOR_PREVIEW_SCROLL_PORTRAIT_KEY}
            id={MDX_EDITOR_PREVIEW_ID_PORTRAIT}
        />
    );
};

MdxEditorPreviewOnly.displayName = "MdxEditorPreviewOnly";
