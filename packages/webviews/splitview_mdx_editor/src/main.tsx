import React from "react";
import "@conundrum/generated/conundrum.css";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
/* import flusterWasm from "@fluster/wasm"; */
import { initializeWebView } from "@fluster/webview_utils";
initializeWebView();

/* eslint-disable-next-line  -- It'll be there... I promise. */
createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
