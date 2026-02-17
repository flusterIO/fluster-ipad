import React, { CSSProperties, useEffect, useRef, type ReactNode } from "react";
import { cn } from "@/utils/cn";
import { useLocalStorage } from "@/state/hooks/use_local_storage";
import { setWebviewWindowBridgeFunctions } from "../state/swift_events/webview_swift_events";
import { LoadingComponent } from "@/shared_components/loading_component";
import { useEventListener } from "@/state/hooks/use_event_listener";
import {
    ScreenDimensions,
    useScreenDimensions,
} from "@/state/hooks/use_screen_dimensions";
import { AnyWebviewAction } from "@/utils/types/any_window_event";
import { WebviewContainerProvider } from "../state/webview_provider";

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
    screenDimensionCalculator,
}: WebViewContainerProps): ReactNode => {
    /* const updateDocSizeTimer = useRef<NodeJS.Timeout | null>(null); */
    const container = useRef<HTMLDivElement>(null);
    const dimensions = useScreenDimensions(screenDimensionCalculator);
    const [darkMode, setDarkMode] = useLocalStorage("dark-mode", undefined, {
        deserializer(value) {
            return value;
        },
        serializer(value) {
            return value;
        },
        initializeWithValue: false,
    });
    useEventListener("set-dark-mode", (e) => {
        if (typeof e.detail === "boolean" && e.detail !== (darkMode === "true")) {
            setDarkMode(e.detail ? "true" : "false");
        }
    });
    useEffect(() => {
        document.body.classList[darkMode === "true" ? "add" : "remove"]("dark")
    }, [darkMode])
    const [theme, setTheme] = useLocalStorage("webview-theme", "fluster", {
        deserializer(value) {
            return value;
        },
        serializer(value) {
            return value;
        },
        initializeWithValue: true,
    });
    useEffect(() => {
        document.body.setAttribute("data-fluster-theme", theme);
    }, [theme]);
    useEventListener("set-webview-theme", (e) => {
        setTheme(e.detail);
    });
    if (darkMode === null) {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center">
                <LoadingComponent />
            </div>
        );
    }
    return (
        <div
            id="webview-container"
            className={cn(
                "max-w-screen w-screen",
                shrinkHeight ? "h-fit" : "h-screen min-h-fit",
                className,
                darkMode === "true" && "dark",
            )}
            style={{
                ...style,
                ...(screenDimensionCalculator &&
                    dimensions && {
                    ...dimensions,
                }),
            }}
        >
            <div
                id="webview-content-wrapper"
                className={cn(
                    "w-full load-hide",
                    shrinkHeight ? "h-fit" : "h-screen min-h-fit",
                    contentContainerClasses,
                )}
                ref={container}
            >
                <WebviewContainerProvider>
                    {children}
                </WebviewContainerProvider>
            </div>
            <div
                id="loading-indicator"
                className="w-full h-full flex flex-col justify-center items-center loading load-show"
            >
                <LoadingComponent className="w-fit h-fit" />
            </div>
        </div>
    );
};

WebViewContainer.displayName = "WebViewContainer";
