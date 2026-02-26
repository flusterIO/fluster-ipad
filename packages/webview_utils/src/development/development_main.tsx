import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { DevelopmentTestPage } from "./development_test_page";
import { setWindowBridgeFunctions } from "#/editor/code_editor/types/swift_events/swift_events";
import { CodeEditorProvider } from "#/editor/code_editor/state/code_editor_provider";
import "../core/styles/base.scss";

setWindowBridgeFunctions();

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <CodeEditorProvider implementation="development">
            <DevelopmentTestPage />
        </CodeEditorProvider>
    </StrictMode>,
);
