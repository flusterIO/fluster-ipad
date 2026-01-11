import React, { useMemo, type ReactNode } from "react";
import { RouterProvider } from "react-router";
import { getDesktopRouter } from "@fluster/webview_utils";

declare global {
    interface Window {
        EXCALIDRAW_ASSET_PATH: string;
    }
}

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
            /* fontURL: ResourceRoutes.mathjaxFonts, */
        },
    };
    window.requestIdleCallback =
        window.requestIdleCallback ||
        function (cb) {
            const start = Date.now();
            return setTimeout(function () {
                cb({
                    didTimeout: false,
                    timeRemaining: function () {
                        return Math.max(0, 50 - (Date.now() - start));
                    },
                });
            }, 1);
        };
    /* useMermaidInit(); */
    /* useGlobalKeymap(); */

    const router = useMemo(() => getDesktopRouter(), []);
    return <RouterProvider router={router} />;
};

App.displayName = "App";

export default App;
