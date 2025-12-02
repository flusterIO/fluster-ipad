import React from "react";
import {
    ResponsiveSplitViewEditor,
    WebViewContainer,
} from "@fluster/webview_utils";

function App() {
    return (
        <WebViewContainer
            style={{
                backgroundColor: "hsl(var(--background))",
            }}
            contentContainerClasses="h-full"
        >
            <ResponsiveSplitViewEditor />
        </WebViewContainer>
    );
}

export default App;
