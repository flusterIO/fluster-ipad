import React, { StrictMode } from "react";
import "@conundrum/main/conundrum.css";
import { createRoot } from "react-dom/client";
import "../../../webview_utils/dist/styles.css";
import "./index.css";
import App from "./App";
import { initializeWebView } from "@fluster/webview_utils";
initializeWebView();

/* eslint-disable-next-line  -- It'll be there. */
createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
