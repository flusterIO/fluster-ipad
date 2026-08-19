import React, { type ReactNode } from "react";
import { LoadingIndicator } from "./loading_indicator";
import { cn } from "@/utils/shad_utils";

export const FullScreenLoading = ({
    message,
}: {
    message?: string;
}): ReactNode => {
    return (
        <div className="w-full h-full min-h-screen flex flex-col justify-center items-center p-4 gap-y-4">
            <LoadingIndicator />
            {message ? (
                <div className="text-sm max-w-112.5 text-foreground">{message}</div>
            ) : null}
        </div>
    );
};

FullScreenLoading.displayName = "FullScreenLoading";

export const CenteredExpandedLoadingIndicator = ({
    className,
}: {
    className?: string;
}): ReactNode => {
    return (
        <div
            className={cn(
                "w-full h-full min-h-fit flex flex-col justify-center items-center",
                className,
            )}
        >
            <LoadingIndicator />
        </div>
    );
};
