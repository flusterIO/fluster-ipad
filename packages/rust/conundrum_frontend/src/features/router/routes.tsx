import React, { type ReactNode } from "react";
import { AnimatePresence } from "framer-motion";
import { Route, Routes, useLocation } from "react-router";
import { AppPaths } from "#/navigation/app_paths";
import { ModularDataDashboard } from "#/dashboard/dashboards/modular_data/modular_data_dashboard";
import { MainSettingsPage } from "#/settings/main_settings_page";
import { WorkspacesPage } from "#/workspace_management/workspaces_page/workspaces_page";
import { ManageSingleWorkspacePage } from "#/workspace_management/manage_single_workspace_page/manage_single_workspace_page"
import { MainFlashcardsPage } from "#/study/flashcards_page/main_flashcards_page";
import { ViewConundrumPage } from "#/cdrm/view_conundrum_page";
import { DatabasePanelPage } from "#/database/database_panel_page";
import {client} from "@/app/rspc_client"

export const MainAppRoutes = (): ReactNode => {
    const location = useLocation();
    return (
        <AnimatePresence>
            <Routes location={location} key={location.pathname}>
                <Route path={AppPaths.dashboard} Component={ModularDataDashboard} />
                <Route path={AppPaths.settings} Component={MainSettingsPage} />
                <Route path={AppPaths.workspaces} Component={WorkspacesPage} />
                <Route path={AppPaths.flashcards} Component={MainFlashcardsPage} />
                <Route path={AppPaths.viewConundrum} Component={ViewConundrumPage} />
                <Route path={AppPaths.database} Component={DatabasePanelPage} />
                <Route
                    path={AppPaths.singleWorkspaceManagement}
                    Component={ManageSingleWorkspacePage} 
                    loader={({params}) => {
                        console.log("params: ", params)
return null
                    }}
                />
            </Routes>
        </AnimatePresence>
    );
};

MainAppRoutes.displayName = "MainAppRoutes";
