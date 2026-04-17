import React from "react";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@conundrum/generated/conundrum.css";
import "./index.css";
import App from "./App";
import {
    NoteDetailWebviewEvents,
    webviewOnError,
} from "@fluster/webview_utils";
import { iniitializeWebView } from "@fluster/webview_utils";
iniitializeWebView();

declare global {
    interface Window {
        setNoteDetails: typeof setNoteDetails;
    }
}

function setNoteDetails(data: number[]) {
    window.dispatchEvent(
        new CustomEvent(NoteDetailWebviewEvents.SetNoteDetails, {
            detail: data,
        }),
    );
}

window.onerror = webviewOnError;

window.setNoteDetails = setNoteDetails;

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
