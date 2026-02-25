import React from "react";
import { BibtexEditor, WebViewContainer } from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";

function App() {
    return (
        <WebViewContainer contentContainerClasses="h-full">
            <BibtexEditor />
        </WebViewContainer>
    );
}

export default App;
