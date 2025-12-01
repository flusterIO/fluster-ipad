import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App";
import {
    applyViewportBroadcastListener,
    sendOrientationSwiftEvents,
    setOrientationListener,
    SwiftHandler,
} from "@fluster/webview_utils";

sendOrientationSwiftEvents();
applyViewportBroadcastListener(
    SwiftHandler.setPreviewViewportSize,
    /* () => document.getElementById("webview-container")!, */
);

window.onload = () => {
    setOrientationListener();
};

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
