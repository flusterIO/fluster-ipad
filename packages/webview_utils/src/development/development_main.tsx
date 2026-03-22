import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { DevelopmentTestPage } from "./development_test_page";
import { setWindowBridgeFunctions } from "#/editor/code_editor/types/swift_events/swift_events";
import "../core/styles/base.scss";
import { WebViewContainer } from "#/webview_container/presentation/webview_container";
import { MdxEditorGlobalProvider } from "#/webview_global_state/provider";
import { createFlusterStore } from "#/webview_global_state/store";

setWindowBridgeFunctions();

const store = createFlusterStore()


/* eslint-disable-next-line  -- It'll be there... */
createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <DevelopmentTestPage />
    </StrictMode>,
);
