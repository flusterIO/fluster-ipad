import React from "react";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { handleSwiftAction } from "@fluster/webview_utils";
import flusterWasm from "@fluster/wasm/fluster";

declare global {
    // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
    interface Window {
        handleSwiftAction: typeof handleSwiftAction;
    }
}

window.handleSwiftAction = handleSwiftAction;
flusterWasm().then((a) => console.log("Wasm Output: ", a));

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
