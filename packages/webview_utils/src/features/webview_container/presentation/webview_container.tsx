import React, { CSSProperties, useEffect, useRef, type ReactNode } from "react";
import { cn } from "@/utils/cn";
import { useLocalStorage } from "@/state/hooks/use_local_storage";
import { setWebviewWindowBridgeFunctions } from "../state/swift_events/webview_swift_events";
import { LoadingComponent } from "@/shared_components/loading_component";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { sendToSwift, SwiftHandler } from "@/utils/bridge/send_to_swift";

interface WebViewContainerProps {
    children: ReactNode;
    className?: string;
    style?: CSSProperties;
    /// If shrinkHeight = true, will shrink to fit-content to allow window to resize to match content
    shrinkHeight?: boolean;
    broadcastHeightKey?: SwiftHandler;
}

setWebviewWindowBridgeFunctions();

export const WebViewContainer = ({
    className,
    children,
    shrinkHeight,
    broadcastHeightKey,
    style,
}: WebViewContainerProps): ReactNode => {
    /* const updateDocSizeTimer = useRef<NodeJS.Timeout | null>(null); */
    const container = useRef<HTMLDivElement>(null);
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
    useEffect(() => {
        if (!broadcastHeightKey) {
            return;
        }
        const em = container.current;
        if (!em) {
            return;
        }
        const h = em.getBoundingClientRect().height;
        if (h) {
            sendToSwift(broadcastHeightKey, h.toString());
        }

        if (!broadcastHeightKey) {
            return;
        }
        const observer = new MutationObserver((mutationsList) => {
            for (const mutation of mutationsList) {
                if (mutation.type === "childList") {
                    /* if (updateDocSizeTimer.current) { */
                    /*     clearTimeout(updateDocSizeTimer.current); */
                    /* } */
                    /* updateDocSizeTimer.current = setTimeout(() => { */
                    /*     const h = container.current?.getBoundingClientRect().height; */
                    /*     if (h) { */
                    /*         sendToSwift(broadcastHeightKey, h.toString()); */
                    /*     } */
                    /* }, 150); */
                    sendToSwift(broadcastHeightKey, h.toString());
                }
            }
        });
        observer.observe(em, {
            childList: true,
        });
    }, []);
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
            ref={container}
            className={cn(
                "max-w-screen",
                shrinkHeight ? "h-fit" : "h-screen min-h-fit",
                className,
                darkMode === "true" && "dark !bg-black",
            )}
            style={style}
        >
            <div id="webview-content-wrapper" className="w-full h-fit load-hide">
                {children}
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
