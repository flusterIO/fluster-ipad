import React, { type HTMLProps, useId, useRef, type ReactNode } from "react";
import { MdxContent } from "./mdx_content";
import { cn } from "@/utils/cn";
import { LoadingComponent } from "@/shared_components/loading_component";
import { type ConundrumState, type EditorState, EditorView, SplitviewEditorDomIds, SplitviewEditorWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { ErrorBoundary } from "react-error-boundary";
import { PreviewLevelErrorReport } from "../error_reporting/preview_level_error_report/preview_level_error_report";
import { type GlobalAppState } from "#/webview_global_state/store";
import { connect } from "react-redux";
import consola from "consola";
import { type WithNullableOptionals } from "../../../core/utils/types/utility_types";
import { ConundrumErrorListener } from "./conundrum_error_listener";


export type MdxEditorPreviewProps = Omit<HTMLProps<HTMLDivElement>, "ref" | "id" | "value">

const connector = connect((state: GlobalAppState) => ({
    parsedValue: state.editor.parsedValue,
    lockEditorScrollToPreview: state.editor.lockEditorScrollToPreview,
    isEditorView: state.editor.editorView === EditorView.Splitview,
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
    isEditorView,
    ...props
}: MdxEditorPreviewProps & Pick<WithNullableOptionals<EditorState>, "lockEditorScrollToPreview" | "parsedValue"> & { isEditorView: boolean }): ReactNode => {
    const ref = useRef<null | HTMLDivElement>(null)
    const id = useId()


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
            <div className="w-screen h-screen min-h-screen top-0 left-0 right-0 bottom-0 fixed flex flex-col justify-center items-center mdx-preview-loading-container">
                <LoadingComponent />
            </div>
        );
    }

    if (parsedValue.trim() === "") {
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
            onError={(e) => { consola.error("Preview boundary error: ", e); }}
            FallbackComponent={(p) => <PreviewLevelErrorReport {...p} mdx={parsedValue} debounceTimeout={0} showWebviewAction={SplitviewEditorWebviewActions.SetWebviewLoaded} id={id} />}
        >
            <ConundrumErrorListener />
            <MdxContent
                id={SplitviewEditorDomIds.MdxPreview}
                {...props}
                ref={ref}
                asMain
                className={cn(
                    "max-w-[1080px]",
                    isEditorView ? "px-6 pt-4 pb-16 mx-auto" : "px-8 pt-6 max-h-screen overflow-y-auto pb-16",
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
