import { WebViewContainer, MdxStandalonePreview } from "@fluster/webview_utils";
import React from "react";

function App() {
    return (
        <WebViewContainer shrinkHeight>
            <MdxStandalonePreview className={"px-6 pb-12 drag-hide"} />
        </WebViewContainer>
    );
}

export default App;
