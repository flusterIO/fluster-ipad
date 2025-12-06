import React, { useState } from "react";
import {
    NoteDetailSheet,
    useWebviewLoadedEvent,
    WebViewContainer,
    NoteDetailWebviewActions,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/webview_utils.css";
import "./index.css";

function App() {
    useWebviewLoadedEvent(NoteDetailWebviewActions.SetWebviewLoaded);
    /* const [data, setData] = useState<MdxParsingResult | null>(null) */
    return (
        <WebViewContainer
            style={{
                backgroundColor: "hsl(var(--background))",
            }}
            contentContainerClasses="h-full"
        >
            <NoteDetailSheet />
        </WebViewContainer>
    );
}

export default App;
