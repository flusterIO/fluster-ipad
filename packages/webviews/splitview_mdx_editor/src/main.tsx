import React from "react";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { handleSwiftAction } from "@fluster/webview_utils";
import flusterWasm from "@fluster/wasm/fluster";

declare global {
    interface Window {
        handleSwiftAction: typeof handleSwiftAction;
    }
}

window.handleSwiftAction = handleSwiftAction;

/* eslint-disable-next-line  -- Calling it a forbidden promise makes it sound worse than it is. */
flusterWasm().then((a) => {
    console.log("Wasm Output: ", a);
});

/* eslint-disable-next-line  -- It'll be there... I promise. */
createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
