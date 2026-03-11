import React, { type HTMLProps, useId, useRef, type ReactNode } from "react";
import { MdxContent } from "./mdx_content";
import { useMediaQuery } from "react-responsive";
import { cn } from "@/utils/cn";
import { LoadingComponent } from "@/shared_components/loading_component";
import { type EditorState, SplitviewEditorDomIds, SplitviewEditorWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { ErrorBoundary } from "react-error-boundary";
import { PreviewLevelErrorReport } from "../error_reporting/preview_level_error_report/preview_level_error_report";
import { type MdxEditorAppState } from "#/webview_global_state/store";
import { connect } from "react-redux";
import consola from "consola";


export type MdxEditorPreviewProps = Omit<HTMLProps<HTMLDivElement>, "ref" | "id" | "value">

const connector = connect((state: MdxEditorAppState) => ({
    parsedValue: state.editor.parsedValue,
    lockEditorScrollToPreview: state.editor.lockEditorScrollToPreview
}))

/**
 * For use _only_ in the primary mdx preview views, either the standalone-preview webview
 * or the splitview editor. 
 * **Do NOT** use this for anything else, as certain state will be inconsistent.
 */
export const MdxEditorPreview = connector(({
    className,
    parsedValue,
    lockEditorScrollToPreview,
    ...props
}: MdxEditorPreviewProps & Pick<EditorState, "lockEditorScrollToPreview" | "parsedValue">): ReactNode => {
    const ref = useRef<null | HTMLDivElement>(null)
    const id = useId()

    const isEditorView = useMediaQuery({
        orientation: "landscape",
    });


    useEventListener("set-editor-scroll-proportion", (e) => {
        // This event will only be sent when the editor is in view,
        // meaning this will never run in portrait or in preview mode.
        if (!ref.current) {
            return
        }
        const newProp = e.detail * (ref.current.scrollHeight - ref.current.clientHeight)

        ref.current.scrollTop = newProp
    })


    if (typeof parsedValue !== "string") {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center mdx-preview-loading-container">
                <LoadingComponent />
            </div>
        );
    }

    if (parsedValue === "") {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center note-content-empty-container">
                <div
                    className="text-xl font-semibold"
                    style={{
                        color: "hsl(var(--foreground))",
                    }}
                >
                    Content Empty
                </div>
            </div>
        );
    }

    return (
        <ErrorBoundary
            onError={(e) => { consola.error("Error: ", e); }}
            FallbackComponent={(p) => <PreviewLevelErrorReport {...p} mdx={parsedValue} debounceTimeout={0} showWebviewAction={SplitviewEditorWebviewActions.SetWebviewLoaded} id={id} />}
        >
            <MdxContent
                id={SplitviewEditorDomIds.MdxPreview}
                {...props}
                ref={ref}
                asMain
                className={cn(
                    "max-w-[1080px]",
                    isEditorView ? "px-6 pt-4 pb-16" : "px-8 pt-6 max-h-screen overflow-y-auto pb-16",
                    className,
                )}
                lockToEditorScroll={lockEditorScrollToPreview}
                mdx={parsedValue}
                showWebviewAction={SplitviewEditorWebviewActions.SetWebviewLoaded}
            />
        </ErrorBoundary>
    );
});

MdxEditorPreview.displayName = "MdxEditorPreview";
