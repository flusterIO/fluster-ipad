import React, { StrictMode } from "react";
import "@conundrum/generated/conundrum.css";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App";
import { iniitializeWebView } from "@fluster/webview_utils";
iniitializeWebView();

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
