import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App";
import {
    applyViewportBroadcastListener,
    SwiftHandler,
} from "@fluster/webview_utils";

applyViewportBroadcastListener(SwiftHandler.setEditorViewportSize);

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
