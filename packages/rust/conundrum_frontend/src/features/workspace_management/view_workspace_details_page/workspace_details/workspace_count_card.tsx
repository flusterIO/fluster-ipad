import React, { type ReactNode } from "react";

interface WorkspaceCountCardProps {
    label: ReactNode;
    count: ReactNode;
}

export const WorkspaceCountCard = ({
    label,
    count,
}: WorkspaceCountCardProps): ReactNode => {
    return (
        <div className="flex flex-col justify-center items-center gap-y-2 w-full bg-fd-card rounded-2xl border p-3">
            <div className="text-lg text-fd-card-foreground/80!">{label}</div>
            <div>{count}</div>
        </div>
    );
};

WorkspaceCountCard.displayName = "WorkspaceCountCard";
