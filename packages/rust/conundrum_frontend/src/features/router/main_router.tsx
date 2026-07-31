import { ModularDataDashboard } from '#/dashboard/dashboards/modular_data/modular_data_dashboard'
import { AppPaths } from '#/navigation/app_paths'
import { MainSettingsPage } from '#/settings/main_settings_page'
import React, { useMemo, type ReactNode } from 'react'
import { createBrowserRouter, type RouteObject, RouterProvider } from "react-router"


const getMainRouter = (): RouteObject[] => {
    return [
        {
            path: AppPaths.dashboard,
            Component: ModularDataDashboard
        },
        {
            path: AppPaths.settings,
            Component: MainSettingsPage
        }
    ]
}


export const MainAppRouter = (): ReactNode => {
    const router = useMemo(() => {
        return createBrowserRouter(getMainRouter())
    }, [])
    return (
        <RouterProvider router={router} />
    )
}


MainAppRouter.displayName = "MainAppRouter"
