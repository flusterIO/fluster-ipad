import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./core/styles/global.scss";
import "@fluster/webview_utils/themes.scss";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
