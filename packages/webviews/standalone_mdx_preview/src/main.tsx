import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "../../../webview_utils/dist/styles.css";
import "./index.css";
import App from "./App";
import { setWindowBridgeFunctions } from "@fluster/webview_utils";

setWindowBridgeFunctions();

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
