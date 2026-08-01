import { cn } from "@/utils/shad_utils";
import React, { type ReactNode } from "react";

interface GridOnLargeProps {
    children: ReactNode;
    className?: string;
}

export const GridOnLarge = ({
    className,
    children,
}: GridOnLargeProps): ReactNode => {
    return (
        <div
            className={cn(
                "w-full flex flex-col justify-start items-start @lg/settings:grid @lg/settings:grid-cols-2 gap-x-4 gap-y-4",
                className,
            )}
        >
            {children}
        </div>
    );
};

GridOnLarge.displayName = "GridOnLarge";
