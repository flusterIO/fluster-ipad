import React, { type ReactNode } from "react";
import { AnimatePresence } from "framer-motion";
import { Route, Routes, useLocation } from "react-router";
import { AppPaths } from "#/navigation/app_paths";
import { ModularDataDashboard } from "#/dashboard/dashboards/modular_data/modular_data_dashboard";
import { MainSettingsPage } from "#/settings/main_settings_page";

export const MainAppRoutes = (): ReactNode => {
    const location = useLocation();
    console.log("location: ", location);
    return (
        <AnimatePresence>
            <Routes location={location} key={location.pathname}>
                <Route path={AppPaths.dashboard} Component={ModularDataDashboard} />
                <Route path={AppPaths.settings} Component={MainSettingsPage} />
            </Routes>
        </AnimatePresence>
    );
};

MainAppRoutes.displayName = "MainAppRoutes";
