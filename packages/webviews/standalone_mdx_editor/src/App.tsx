import {
    applyViewportBroadcastListener,
    CodeEditor,
    SwiftHandler,
    WebViewContainer,
} from "@fluster/webview_utils";
import React from "react";

applyViewportBroadcastListener(SwiftHandler.setEditorViewportSize);
window.onload = () => {
    window.requestDocumentSize()
};

function App() {
    return (
        <WebViewContainer>
            <CodeEditor />
        </WebViewContainer>
    );
}

export default App;
