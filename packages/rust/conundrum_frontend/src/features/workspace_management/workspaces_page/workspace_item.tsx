import React, { type ReactNode } from "react";

interface WorkspaceListItemProps {
    workspace: object;
}

export const WorkspaceListItem = (props: WorkspaceListItemProps): ReactNode => {
    return <div>Workspace here</div>;
};

WorkspaceListItem.displayName = "WorkspaceListItem";
