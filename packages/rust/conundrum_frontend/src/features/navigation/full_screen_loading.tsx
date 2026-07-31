import React, { type ReactNode } from "react";
import { LoadingIndicator } from "./loading_indicator";

export const FullScreenLoading = ({
    message,
}: {
    message?: string;
}): ReactNode => {
    return (
        <div className="w-full h-full min-h-screen flex flex-col justify-center items-center p-4 gap-y-4">
            <LoadingIndicator />
            {message ? <div className="text-sm max-w-112.5">{message}</div> : null}
        </div>
    );
};

FullScreenLoading.displayName = "FullScreenLoading";
