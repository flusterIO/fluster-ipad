import React from "react";
import { BibtexEditor, WebViewContainer } from "@fluster/webview_utils";
import "../../../webview_utils/dist/webview_utils.css";
import "./index.css";

function App() {
    return (
        <WebViewContainer
            style={{
                backgroundColor: "hsl(var(--background))",
            }}
            contentContainerClasses="h-full"
        >
            <BibtexEditor />
        </WebViewContainer>
    );
}

export default App;
