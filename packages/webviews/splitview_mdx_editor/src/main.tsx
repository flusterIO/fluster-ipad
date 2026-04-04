import React from "react";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import flusterWasm from "@fluster/wasm";
import { iniitializeWebView } from "@fluster/webview_utils";
iniitializeWebView();

void flusterWasm().then(() => {
    console.log("Wasm loaded...");
});

/* eslint-disable-next-line  -- It'll be there... I promise. */
createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
