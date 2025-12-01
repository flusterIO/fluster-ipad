import {
    WebViewContainer,
    MdxStandalonePreview,
    SwiftHandler,
} from "@fluster/webview_utils";
import React from "react";

function App() {
    return (
        <WebViewContainer broadcastHeightKey={SwiftHandler.setPreviewViewportSize}>
            <MdxStandalonePreview className={"px-6 pb-12 pt-8"} />
        </WebViewContainer>
    );
}

export default App;
