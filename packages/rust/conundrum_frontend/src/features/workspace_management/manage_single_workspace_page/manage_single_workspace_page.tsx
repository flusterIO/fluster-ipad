import React, { type ReactNode } from "react";
import { PageContainer } from "@/components/general/page_container";
import { WorkspaceLoader } from "../workspaces_page/workspace_loader";
import { WorkspaceForm } from "./workspace_form/workspace_form";

export const ManageSingleWorkspacePage = (): ReactNode => {
    return (
        <PageContainer>
            <WorkspaceLoader>
                <WorkspaceForm />
            </WorkspaceLoader>
        </PageContainer>
    );
};

ManageSingleWorkspacePage.displayName = "ManageSingleWorkspacePage";
