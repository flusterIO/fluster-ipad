import React, { type ReactNode } from "react";
import { MdxEditorPreview } from "./mdx_editor_preview";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { useCodeEditorDispatch } from "#/editor/code_editor/state/code_editor_provider";

/** A utility component used to implement some event listeners before rendering the MdxEditorPreview component. */
export const MdxEditorPreviewOnly = (): ReactNode => {
    const dispatch = useCodeEditorDispatch();
    useEventListener("set-editor-content", (e) => {
        dispatch({
            type: "setValue",
            payload: e.detail,
        });
    });
    return <MdxEditorPreview />;
};

MdxEditorPreviewOnly.displayName = "MdxEditorPreviewOnly";
