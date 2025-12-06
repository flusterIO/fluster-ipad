import { WebViewContainer, MdxStandalonePreview } from "@fluster/webview_utils";
import React from "react";

function App() {
    return (
        <WebViewContainer>
            <MdxStandalonePreview className={"px-6 pb-12 pt-8"} />
        </WebViewContainer>
    );
}

export default App;
