import React, { CSSProperties, useEffect, type ReactNode } from "react";
import { cn } from "@/utils/cn";
import { useLocalStorage } from "@/state/hooks/use_local_storage";
import { setWebviewWindowBridgeFunctions } from "../state/swift_events/webview_swift_events";
import { LoadingComponent } from "@/shared_components/loading_component";
import { useEventListener } from "@/state/hooks/use_event_listener";

interface WebViewContainerProps {
    children: ReactNode;
    className?: string;
    style?: CSSProperties;
}

setWebviewWindowBridgeFunctions();

export const WebViewContainer = ({
    className,
    children,
    style,
}: WebViewContainerProps): ReactNode => {
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
        console.log("e.detail: ", e.detail);
        if (typeof e.detail === "boolean" && e.detail !== (darkMode === "true")) {
            setDarkMode(e.detail ? "true" : "false");
        }
    });
    if (darkMode === null) {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center">
                <LoadingComponent />
            </div>
        );
    }
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
    return (
        <div
            className={cn(
                "w-screen h-screen",
                className,
                darkMode === "true" && "dark",
            )}
            style={style}
        >
            {children}
        </div>
    );
};

WebViewContainer.displayName = "WebViewContainer";
