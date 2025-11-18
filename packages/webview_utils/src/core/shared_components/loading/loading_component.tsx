import { cn } from "@/utils/cn";
import React, { type ReactNode } from "react";

interface LoadingComponentProps {
    className?: string;
}

export const LoadingComponent = (props: LoadingComponentProps): ReactNode => {
    return (
        <div
            className={cn(
                "w-full h-full flex flex-col justify-center items-center text-center",
                props.className,
            )}
        >
            loading...
        </div>
    );
};

LoadingComponent.displayName = "LoadingComponent";
