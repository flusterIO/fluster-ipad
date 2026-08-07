import React, { type ReactNode } from "react";
import { AnimatePresence } from "framer-motion";
import { Route, RouteProps, Routes, useLocation, data } from "react-router";
import { AppPaths } from "#/navigation/app_paths";
import { ModularDataDashboard } from "#/dashboard/dashboards/modular_data/modular_data_dashboard";
import { MainSettingsPage } from "#/settings/main_settings_page";
import { WorkspacesPage } from "#/workspace_management/workspaces_page/workspaces_page";
import { ManageSingleWorkspacePage } from "#/workspace_management/manage_single_workspace_page/manage_single_workspace_page";
import { MainFlashcardsPage } from "#/study/flashcards_page/main_flashcards_page";
import { ViewConundrumPage } from "#/cdrm/view_conundrum_page";
import { DatabasePanelPage } from "#/database/database_panel_page";
import { client } from "@/app/rspc_client";
import { ViewWorkspaceDetailsPage } from "#/workspace_management/view_workspace_details_page/view_workspace_details_page";
import { RouteErrorBoundary } from "#/error_handling/components/route_error_boundary";
import { logMaybeObject } from "#/error_handling/utils/log_maybe_object";

export const MainAppRoutes = (): ReactNode => {
    const location = useLocation();
    return (
        <AnimatePresence>
            <Routes location={location} key={location.pathname}>
                <Route
                    path={AppPaths.dashboard}
                    Component={ModularDataDashboard}
                    index
                />
                <Route path={AppPaths.settings} Component={MainSettingsPage} />
                <Route path={AppPaths.flashcards} Component={MainFlashcardsPage} />
                <Route path={AppPaths.viewConundrum} Component={ViewConundrumPage} />
                <Route path={AppPaths.database} Component={DatabasePanelPage} />
                <Route
                    path={AppPaths.singleWorkspaceManagement}
                    Component={ManageSingleWorkspacePage}
                    loader={({ params }) => {
                        console.log("params: ", params);
                        return null;
                    }}
                    index={false}
                />
                <Route
                    path={AppPaths.singleWorkspaceView}
                    Component={ViewWorkspaceDetailsPage}
                    index={false}
                    id={AppPaths.singleWorkspaceView}
                    loader={async ({ params }) => {
                        console.log("params: ", params);
                        const fsPath = params.fs_path;
                        console.log("fsPath: ", fsPath);
                        if (!fsPath) {
                            // eslint-disable-next-line @typescript-eslint/only-throw-error
                            throw data("Workspace path was not provided.", { status: 404 });
                        }
                        console.log("fsPath: ", fsPath);
                        try {
                            const res = await client.query([
                                "user_workspace_crud.get_by_predicate",
                                {
                                    predicate: `root="${fsPath}"`,
                                    pagination: {
                                        page: 1,
                                        per_page: 1,
                                    },
                                },
                            ]);
                            return res;
                        } catch (err: unknown) {
                            logMaybeObject("Error: ", err);
                        }
                    }}
                    ErrorBoundary={RouteErrorBoundary}
                />
                <Route path={AppPaths.workspaces} Component={WorkspacesPage} index />
            </Routes>
        </AnimatePresence>
    );
};

MainAppRoutes.displayName = "MainAppRoutes";
