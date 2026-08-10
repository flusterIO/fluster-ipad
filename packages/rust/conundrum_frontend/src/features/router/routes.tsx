import React, { type ReactNode } from "react";
import { AnimatePresence } from "framer-motion";
import { Route, Routes, useLocation } from "react-router";
import { AppPaths } from "#/navigation/app_paths";
import { ModularDataDashboard } from "#/dashboard/dashboards/modular_data/modular_data_dashboard";
import { MainSettingsPage } from "#/settings/main_settings_page";
import { WorkspacesPage } from "#/workspace_management/workspaces_page/workspaces_page";
import { ManageSingleWorkspacePage } from "#/workspace_management/manage_single_workspace_page/manage_single_workspace_page";
import { MainFlashcardsPage } from "#/study/flashcards_page/main_flashcards_page";
import { ViewConundrumPage } from "#/cdrm/view_conundrum_page";
import { ViewWorkspaceDetailsPage } from "#/workspace_management/view_workspace_details_page/view_workspace_details_page";
import { RouteErrorBoundary } from "#/error_handling/components/route_error_boundary";
import { DatabaseTablePage } from "#/database/database_table/database_table_page";
import { HealthPage } from "#/health/health_page";

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
                <Route path={AppPaths.database} Component={DatabaseTablePage} />
                <Route
                    path={AppPaths.singleWorkspaceManagement}
                    Component={ManageSingleWorkspacePage}
                    index={false}
                />
                <Route
                    path={AppPaths.singleWorkspaceView}
                    Component={ViewWorkspaceDetailsPage}
                    index={false}
                    id={AppPaths.singleWorkspaceView}
                    /* loader={async ({ params }) => { */
                    /*     console.log("params: ", params); */
                    /*     const fsPath = params.fs_path; */
                    /*     console.log("fsPath: ", fsPath); */
                    /* }} */
                    ErrorBoundary={RouteErrorBoundary}
                />
                <Route path={AppPaths.workspaces} Component={WorkspacesPage} index />
                <Route path={AppPaths.health} Component={HealthPage} />
            </Routes>
        </AnimatePresence>
    );
};

MainAppRoutes.displayName = "MainAppRoutes";
