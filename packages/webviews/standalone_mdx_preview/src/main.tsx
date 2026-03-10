import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "../../../webview_utils/dist/styles.css";
import "./index.css";
import App from "./App";

/* eslint-disable-next-line  -- It'll be there. */
createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
