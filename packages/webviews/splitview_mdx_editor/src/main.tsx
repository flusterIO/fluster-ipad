import React from "react";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { handleSwiftAction } from "@fluster/webview_utils";
import flusterWasm from "@fluster/wasm";

declare global {
    interface Window {
        handleSwiftAction: typeof handleSwiftAction;
    }
}

window.handleSwiftAction = handleSwiftAction;

void flusterWasm().then(() => {
    console.log("Wasm loaded...");
});

/* eslint-disable-next-line  -- It'll be there... I promise. */
createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
