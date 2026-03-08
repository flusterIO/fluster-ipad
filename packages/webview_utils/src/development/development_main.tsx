import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { DevelopmentTestPage } from "./development_test_page";
import { setWindowBridgeFunctions } from "#/editor/code_editor/types/swift_events/swift_events";
import "../core/styles/base.scss";
import { WebViewContainer } from "#/webview_container/presentation/webview_container";
import { MdxEditorGlobalProvider } from "#/webview_global_state/mdx_editor/provider";
import { createFlusterStore } from "#/webview_global_state/mdx_editor/store";
import { WebviewEnvironment } from "@/code_gen/typeshare/fluster_core_utilities";

setWindowBridgeFunctions();

const store = createFlusterStore(WebviewEnvironment.MacOS)


createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <WebViewContainer>
            <MdxEditorGlobalProvider {...store}>
                <DevelopmentTestPage />
            </MdxEditorGlobalProvider>
        </WebViewContainer>
    </StrictMode>,
);
