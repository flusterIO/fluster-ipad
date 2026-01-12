import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./core/styles/main.scss";
import { MainDesktopProvider } from "@fluster/webview_utils";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <MainDesktopProvider>
            <App />
        </MainDesktopProvider>
    </React.StrictMode>,
);
