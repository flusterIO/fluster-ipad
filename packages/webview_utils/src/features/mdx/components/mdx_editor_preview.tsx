import { useCodeEditorContext } from "#/editor/code_editor/state/code_editor_provider";
import React, { HTMLProps, useEffect, type ReactNode } from "react";
import { MdxContent } from "./mdx_content";
import { useMediaQuery } from "react-responsive";
import { cn } from "@/utils/cn";
import { LoadingComponent } from "@/shared_components/loading_component";
import { SplitviewEditorWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { AnyWebviewStorageKey } from "@/utils/types/any_window_event";
import { ErrorBoundary } from "react-error-boundary";
import { MdxParsingErrorComponent } from "./mdx_parsing_error_component";
import { setBodyLoading } from "./editor_dom_utils";
import { setMdxPreviewWindowMethods } from "./standalone_mdx_preview/standalone_mdx_preview_swift_events";
import { setWindowBridgeFunctions } from "#/editor/code_editor/types/swift_events/swift_events";


setMdxPreviewWindowMethods();

setWindowBridgeFunctions();

export type MdxEditorPreviewProps = HTMLProps<HTMLDivElement> &
{
    scrollPositionKeyLandscape: AnyWebviewStorageKey
    scrollPositionKeyPortrait: AnyWebviewStorageKey
};

export const MdxEditorPreview = ({
    className,
    scrollPositionKeyPortrait,
    scrollPositionKeyLandscape,
    ...props
}: MdxEditorPreviewProps): ReactNode => {
    const { value, parsedValue } = useCodeEditorContext();

    const isEditorView = useMediaQuery({
        orientation: "landscape",
    });

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
                scrollPositionKey={isEditorView ? scrollPositionKeyLandscape : scrollPositionKeyPortrait}
                className={cn(
                    "max-w-[1080px]",
                    isEditorView ? "px-6 pt-4 pb-16" : "px-8 pt-6 max-h-screen overflow-y-auto pb-16",
                    className,
                )}
                mdx={parsedValue}
                showWebviewAction={SplitviewEditorWebviewActions.SetWebviewLoaded}
            />
        </ErrorBoundary>
    );
};

MdxEditorPreview.displayName = "MdxEditorPreview";
