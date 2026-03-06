import React from "react";
import {
    BibtexEditor,
    MdxEditorGlobalProvider,
    WebViewContainer,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";

function App() {
    return (
        <WebViewContainer contentContainerClasses="h-full">
            <MdxEditorGlobalProvider>
                <BibtexEditor />
            </MdxEditorGlobalProvider>
        </WebViewContainer>
    );
}

export default App;
