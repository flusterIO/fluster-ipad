import React, { type ReactNode } from "react";
import { SyncLoader } from "react-spinners";

interface LoadingComponentProps {
    className?: string;
    size?: number;
}

export const LoadingComponent = ({
    className,
    size = 16,
}: LoadingComponentProps): ReactNode => {
    return (
        <SyncLoader
            className={className}
            loading={true}
            size={size}
            color="hsl(var(--primary))"
        />
    );
};

LoadingComponent.displayName = "LoadingComponent";
