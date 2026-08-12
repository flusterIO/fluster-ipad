import React, { type ReactNode } from "react";
import { WorkspaceDetailsView } from "./workspace_details/workspace_details";
import { WorkspaceLoader } from "../workspaces_page/workspace_loader";

export const ViewWorkspaceDetailsPage = (): ReactNode => {
    return (
        <WorkspaceLoader>
            <WorkspaceDetailsView />
        </WorkspaceLoader>
    );
};

ViewWorkspaceDetailsPage.displayName = "ViewWorkspaceDetailsPage";
