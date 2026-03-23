import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { DevelopmentTestPage } from "./development_test_page";
import "../core/styles/base.scss";



/* eslint-disable-next-line  -- It'll be there... */
createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <DevelopmentTestPage />
    </StrictMode>,
);
