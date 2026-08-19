import React, { StrictMode } from "react";
import "@conundrum/main/conundrum.css";
import "@conundrum/ts/katex-css-without-fonts.css";
import { createRoot } from "react-dom/client";
import "./index.css";
import { App } from "@/app/app";
import { KatexFontLoader } from "@conundrum/ts/architecture"
import { initializeConundrumWeb } from "@conundrum/ts"

initializeConundrumWeb()

/* eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- It'll be there... */
createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
        <KatexFontLoader fontUrl="/" />
    </StrictMode>,
);
