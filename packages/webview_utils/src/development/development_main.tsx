import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { DevelopmentTestPage } from "./development_test_page";
import { setWindowBridgeFunctions } from "#/editor/code_editor/types/swift_events/swift_events";
import { CodeEditorProvider } from "#/editor/code_editor/state/code_editor_provider";
import "../core/styles/base.scss";
import { WebViewContainer } from "#/webview_container/presentation/webview_container";

setWindowBridgeFunctions();

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <WebViewContainer>
            <CodeEditorProvider implementation="development">
                <DevelopmentTestPage />
            </CodeEditorProvider>
        </WebViewContainer>
    </StrictMode>,
);
