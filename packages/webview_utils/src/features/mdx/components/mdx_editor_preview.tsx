import { useCodeEditorContext } from "#/editor/code_editor/state/code_editor_provider";
import React, { HTMLProps, useEffect, useRef, type ReactNode } from "react";
import { MdxContent } from "./mdx_content";
import { useMediaQuery } from "react-responsive";
import { cn } from "@/utils/cn";
import { LoadingComponent } from "@/shared_components/loading_component";
import { SplitviewEditorWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { ErrorBoundary } from "react-error-boundary";
import { MdxParsingErrorComponent } from "./mdx_parsing_error_component";
import { setBodyLoading } from "./editor_dom_utils";
import { setMdxPreviewWindowMethods } from "./standalone_mdx_preview/standalone_mdx_preview_swift_events";
import { setWindowBridgeFunctions } from "#/editor/code_editor/types/swift_events/swift_events";
import { useEventListener } from "@/state/hooks/use_event_listener";


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
        if (parsedValue !== null) {
            setBodyLoading(false)
        }
    }, [parsedValue, value])


    if (value === "" && parsedValue === "") {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center">
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

    if (!parsedValue || parsedValue === "") {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center">
                <LoadingComponent />
            </div>
        );
    }

    return (
        <ErrorBoundary
            FallbackComponent={MdxParsingErrorComponent}
            onError={(e) => console.error("Error Boundary: ", e)}
        >
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
        </ErrorBoundary>
    );
};

MdxEditorPreview.displayName = "MdxEditorPreview";
