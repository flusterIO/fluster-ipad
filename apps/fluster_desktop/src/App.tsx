import { type ReactNode } from "react";

import { BrowserRouter, Route, Routes } from "react-router";
import {
    AppRoutes,
    ResourceRoutes,
    SplashScreen,
    DashboardPage,
    DesktopScaffold,
} from "@fluster/webview_utils";

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
    /* useMermaidInit(); */
    /* useGlobalKeymap(); */

    /* const router = useMemo(() => getDesktopRouter(), [])/* ; */
    /* return <RouterProvider router={router} />; */
    return (
        <BrowserRouter>
            <Routes>
                <Route path={AppRoutes.splash_screen} element={<SplashScreen />} />
                <Route element={<DesktopScaffold />}>
                    <Route index path={AppRoutes.dashboard} element={<DashboardPage />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
};

App.displayName = "App";

export default App;
