import { CodeEditor, WebViewContainer } from "@fluster/webview_utils";
import React from "react";

function App() {
    return (
        <WebViewContainer
        /* shrinkHeight */
        >
            <CodeEditor />
        </WebViewContainer>
    );
}

export default App;
