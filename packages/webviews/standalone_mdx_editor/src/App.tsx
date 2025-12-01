import {
    CodeEditor,
    SwiftHandler,
    WebViewContainer,
} from "@fluster/webview_utils";
import React from "react";

function App() {
    return (
        <WebViewContainer
            shrinkHeight
            broadcastHeightKey={SwiftHandler.setEditorViewportSize}
        >
            <CodeEditor />
        </WebViewContainer>
    );
}

export default App;
