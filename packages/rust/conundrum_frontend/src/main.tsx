import React, { StrictMode } from "react";
import "@conundrum/main/conundrum.css";
import { createRoot } from "react-dom/client";
import "./index.css";
import { App } from "@/app/app";
import ReduxProvider from "@/state/redux_provider";

/* eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- It'll be there... */
createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <ReduxProvider>
            <App />
        </ReduxProvider>
    </StrictMode>,
);
