import React, { useMemo, type ReactNode } from "react";
import { AppRoutes } from "./features/router/data/app_routes";
import { ResourceRoutes } from "./features/router/data/resource_routes";
import { MainDesktopScaffold } from "./features/scaffold/presentation/main_desktop_scaffold";
import { DashboardSwitch } from "./features/dashboard/presentation/dashboard_switch";
import { createBrowserRouter, RouterProvider } from "react-router";
import ReduxProvider from "@/state/redux_provider";
import { SplashScreen } from "@fluster/webview_utils";
import { MdxNotePage } from "#/mdx/presentation/mdx_note_page/mdx_note_page";
import { CommandPaletteContainer } from "#/command_palette/presentation/command_palette_container";
import { useGlobalKeymap } from "#/keymap/state/hooks/use_global_keymap";
import { GlobalKeymapListener } from "#/keymap/presentation/global_keymap_listener";

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
                    {
                        path: AppRoutes.mdxNote,
                        children: [
                            {
                                path: ":fsPath",
                                Component: MdxNotePage,
                                loader: async ({ params }) => {
                                    console.log("params: ", params);
                                },
                            },
                        ],
                    },
                ],
            },
        ]);
    }, []);
    /* useMermaidInit(); */

    return (
        <ReduxProvider>
            <GlobalKeymapListener />
            <RouterProvider router={router} />
            <CommandPaletteContainer />
        </ReduxProvider>
    );
};

App.displayName = "App";

export default App;
