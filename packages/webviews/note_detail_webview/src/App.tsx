import React from "react";
import {
    LoadingComponent,
    NoteDetailSheet,
    WebViewContainer,
} from "@fluster/webview_utils";
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
            <NoteDetailSheet />
            <div className="w-full h-full flex flex-col justify-center items-center loading-show">
                <LoadingComponent />
            </div>
        </WebViewContainer>
    );
}

export default App;
