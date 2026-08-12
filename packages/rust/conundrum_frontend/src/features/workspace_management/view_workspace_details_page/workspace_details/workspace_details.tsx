import { type WorkspaceByPredicate } from "#/database/db_utility_types/workspace";
import { useGenericRemoteDataContext } from "#/database/state/generic_data_loading_context/generic_data_loading_context";
import { RouteError404 } from "#/error_handling/components/route_error_404";
import { CenteredExpandedLoadingIndicator } from "#/navigation/full_screen_loading";
import { DisabledLabeledText } from "#/settings/inputs/string_inputs/disabled_labeled_text";
import { PageContainer } from "@/components/general/page_container";
import React, { type ReactNode } from "react";
import { WorkspaceCount } from "./workspace_count";
import { Link } from "react-router";
import { getWorkspaceManagementPath } from "#/workspace_management/path_utils/get_workspace_item_path";
import { buttonVariants } from "@/components/shad/button";

interface WorkspaceDetailsViewProps {
    workspace: WorkspaceByPredicate;
}

export const WorkspaceDetailsView = (): ReactNode => {
    const { data, loading } =
        useGenericRemoteDataContext<WorkspaceDetailsViewProps>();
    if (loading) {
        return <CenteredExpandedLoadingIndicator />;
    }
    if (!data) {
        return <RouteError404 expand />;
    }
    if (!data.workspace) {
        return <RouteError404 expand />;
    }
    const { workspace } = data;
    return (
        <PageContainer title="Workspace Details">
            <div className="@container/workspaceDetails w-full max-w-270 py-6">
                <div className="grid grid-cols-1 @[640px]/workspaceDetails:grid-cols-2 gap-x-4">
                    <DisabledLabeledText
                        label="Label"
                        content={workspace.label ?? "--"}
                    />
                    <DisabledLabeledText label="Root" content={workspace.root} mono />
                </div>
                <WorkspaceCount workspace={workspace} />
                <div className="w-full flex flex-row justify-end items-center">
                    <Link
                        to={getWorkspaceManagementPath(workspace.root)}
                        className={buttonVariants()}
                    >
                        Edit Workspace
                    </Link>
                </div>
            </div>
        </PageContainer>
    );
};

WorkspaceDetailsView.displayName = "WorkspaceDetailsView";
