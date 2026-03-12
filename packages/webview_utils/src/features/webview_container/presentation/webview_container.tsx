import React, { type CSSProperties, type ReactNode } from "react";
import { cn } from "@/utils/cn";
import { setWebviewWindowBridgeFunctions } from "../state/swift_events/webview_swift_events";
import { LoadingComponent } from "@/shared_components/loading_component";
import {
    type ScreenDimensions,
} from "@/state/hooks/use_screen_dimensions";
import { type AnyWebviewAction } from "@/utils/types/any_window_event";

interface WebViewContainerProps {
    children: ReactNode;
    className?: string;
    style?: CSSProperties;
    contentContainerClasses?: string;
    /** If shrinkHeight = true, will shrink to fit-content to allow window to resize to match content */
    shrinkHeight?: boolean;
    broadcastHeightKey?: AnyWebviewAction;
    /** An optional function that can accept the actual screen dimensions sent by swift and returns another screen dimensions that the webview will bind it's size to. */
    screenDimensionCalculator?: (
        actualDimensions: ScreenDimensions,
    ) => CSSProperties;
}

setWebviewWindowBridgeFunctions();

/** A utility function intended to have a shared place to get this element since this webview keeps breaking. */
export const getParentContentWrapper = () =>
    document.getElementById("webview-content-wrapper");



export const WebViewContainer = ({
    className,
    children,
    shrinkHeight,
    style,
    contentContainerClasses,
}: WebViewContainerProps): ReactNode => {


    return (
        <div
            id="webview-container"
            className={cn(
                "max-w-screen w-screen",
                shrinkHeight ? "h-fit" : "h-fit min-h-screen",
                className,
            )}
            style={style}
        >
            <div
                id="webview-content-wrapper"
                className={cn(
                    "w-full load-hide",
                    shrinkHeight ? "h-fit" : "h-fit min-h-screen",
                    contentContainerClasses,
                )}
            >
                {children}
            </div>
            <div
                id="loading-indicator"
                className="w-screen h-screen flex flex-col justify-center items-center loading load-show fixed top-0 left-0 right-0 bottom-0"
            >
                <LoadingComponent className="w-fit h-fit" />
            </div>
        </div >
    );
};

WebViewContainer.displayName = "WebViewContainer";
