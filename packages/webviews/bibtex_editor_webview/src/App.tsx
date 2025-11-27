import React from "react";
import { BibtexEditor, WebViewContainer } from "@fluster/webview_utils";

function App() {
    return (
        <WebViewContainer
            style={{
                backgroundColor: "hsl(var(--background))",
            }}
        >
            <BibtexEditor />
        </WebViewContainer>
    );
}

export default App;
