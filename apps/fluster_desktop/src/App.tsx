import React, { useMemo, type ReactNode } from "react";
import { AppRoutes } from "./features/router/data/app_routes";
import { ResourceRoutes } from "./features/router/data/resource_routes";
import { MainDesktopScaffold } from "./features/scaffold/presentation/main_desktop_scaffold";
import { DashboardSwitch } from "./features/dashboard/presentation/dashboard_switch";
import { createBrowserRouter, RouterProvider } from "react-router";
import ReduxProvider from "@/state/redux_provider";
import { SplashScreen } from "@fluster/webview_utils";

const App = (): ReactNode => {
    window.MathJax = {
        /* @ts-expect-error -- Not sure if this is working but I'm leaving it until all math is rendering properly. */
        "HTML-CSS": { linebreaks: { automatic: true } },
        tex: {
            inlineMath: [["$", "$"]],
        },
        menuSettings: {
            autocollapse: true,
        },
        chtml: {
            minScale: 0.2,
            fontURL: ResourceRoutes.mathjaxFonts,
        },
    };

    const router = useMemo(() => {
        return createBrowserRouter([
            {
                path: AppRoutes.splashScreen,
                Component: SplashScreen,
            },
            {
                Component: MainDesktopScaffold,
                children: [
                    {
                        path: AppRoutes.dashboard,
                        Component: DashboardSwitch,
                    },
                ],
            },
        ]);
    }, []);
    /* useMermaidInit(); */
    /* useGlobalKeymap(); */

    return (
        <ReduxProvider>
            <RouterProvider router={router} />;
        </ReduxProvider>
    );
};

App.displayName = "App";

export default App;
