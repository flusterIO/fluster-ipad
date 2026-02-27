import { useCodeEditorContext } from "#/editor/code_editor/state/code_editor_provider";
import React, { HTMLProps, useEffect, useRef, type ReactNode } from "react";
import { MdxContent } from "./mdx_content";
import { useMediaQuery } from "react-responsive";
import { cn } from "@/utils/cn";
import { LoadingComponent } from "@/shared_components/loading_component";
import { MdxPreviewWebviewActions, SplitviewEditorWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { setBodyLoading } from "./editor_dom_utils";
import { setMdxPreviewWindowMethods } from "./standalone_mdx_preview/standalone_mdx_preview_swift_events";
import { setWindowBridgeFunctions } from "#/editor/code_editor/types/swift_events/swift_events";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { sendToSwift } from "@/utils/bridge/send_to_swift";


setMdxPreviewWindowMethods();

setWindowBridgeFunctions();

export type MdxEditorPreviewProps = Omit<HTMLProps<HTMLDivElement>, "ref">

export const MdxEditorPreview = ({
    className,
    ...props
}: MdxEditorPreviewProps): ReactNode => {
    const { value, parsedValue, lockEditorScrollToPreview } = useCodeEditorContext();
    const ref = useRef<null | HTMLDivElement>(null)

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

    useEffect(() => {
        if (typeof parsedValue === "string") {
            setBodyLoading(false)
        } else {
            sendToSwift(MdxPreviewWebviewActions.RequestNoteData)
        }
    }, [parsedValue, value])


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
        <MdxContent
            id="mdx-preview"
            {...props}
            ref={ref}
            className={cn(
                "max-w-[1080px]",
                isEditorView ? "px-6 pt-4 pb-16" : "px-8 pt-6 max-h-screen overflow-y-auto pb-16",
                className,
            )}
            lockToEditorScroll={lockEditorScrollToPreview}
            mdx={parsedValue}
            showWebviewAction={SplitviewEditorWebviewActions.SetWebviewLoaded}
        />
    );
};

MdxEditorPreview.displayName = "MdxEditorPreview";
