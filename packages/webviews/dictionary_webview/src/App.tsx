import React from "react";
import {
    DictionaryPage,
    useDictionaryData,
    setMdxBridgeFunctions,
    WebViewContainer,
    LoadingComponent,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/webview_utils.css";
import "./index.css";

setMdxBridgeFunctions();

function App() {
    const data = useDictionaryData();
    return (
        <WebViewContainer
            style={{
                backgroundColor: "hsl(var(--background))",
            }}
            contentContainerClasses="h-full"
        >
            {data ? (
                <DictionaryPage data={data} />
            ) : (
                <div className="w-full h-full flex flex-col justfy-center items-center">
                    <LoadingComponent />
                </div>
            )}
        </WebViewContainer>
    );
}

export default App;
