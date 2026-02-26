import React from "react";
import {
    ResponsiveSplitViewEditor,
    WebViewContainer,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";

function App() {
    return (
        <WebViewContainer contentContainerClasses="h-full">
            <ResponsiveSplitViewEditor />
        </WebViewContainer>
    );
}

export default App;
