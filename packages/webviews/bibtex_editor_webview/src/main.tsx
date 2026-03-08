import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App";
import { handleSwiftAction } from "@fluster/webview_utils";
import flusterWasm from "@fluster/wasm";

declare global {
    interface Window {
        handleSwiftAction: typeof handleSwiftAction;
    }
}

window.handleSwiftAction = handleSwiftAction;
void flusterWasm().then((a) => {
    console.log("Wasm Output: ", a);
});

/* eslint-disable-next-line  -- It'll be there, I promise. */
createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
