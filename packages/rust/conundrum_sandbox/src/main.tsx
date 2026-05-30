import React, { StrictMode } from "react";
import "@conundrum/generated/conundrum.css";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App";
import { initializeWebView } from "@fluster/webview_utils";
initializeWebView();

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
