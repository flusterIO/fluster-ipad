import React, { CSSProperties, type ReactNode } from "react";
import { cn } from "@/utils/cn";
import { useLocalStorage } from "@/state/hooks/use_local_storage";

interface WebViewContainerProps {
    children: ReactNode;
    className?: string;
    style?: CSSProperties;
}

export const WebViewContainer = ({
    className,
    children,
    style,
}: WebViewContainerProps): ReactNode => {
    const [darkMode] = useLocalStorage("darkMode");
    return (
        <div
            className={cn(
                "w-screen h-screen",
                className,
                [true, "true"].includes(darkMode as string) && "dark",
            )}
            style={style}
        >
            {children}
        </div>
    );
};

WebViewContainer.displayName = "WebViewContainer";
