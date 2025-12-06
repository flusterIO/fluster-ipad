import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App";
import {
    sendOrientationSwiftEvents,
    setOrientationListener,
} from "@fluster/webview_utils";

sendOrientationSwiftEvents();

window.onload = () => {
    setOrientationListener();
};

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
