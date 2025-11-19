import React, { type ReactNode } from "react";
import { cn } from "@/utils/cn";
import { useLocalStorage } from "@/state/hooks/use_local_storage";

interface WebViewContainerProps {
    children: ReactNode;
    className?: string;
}

export const WebViewContainer = ({
    className,
    children,
}: WebViewContainerProps): ReactNode => {
    const [darkMode] = useLocalStorage("darkMode");
    console.log("darkMode: ", darkMode);
    return (
        <div
            className={cn(
                "w-screen h-screen",
                className,
                [true, "true"].includes(darkMode as string) && "dark",
            )}
        >
            {children}
        </div>
    );
};

WebViewContainer.displayName = "WebViewContainer";
