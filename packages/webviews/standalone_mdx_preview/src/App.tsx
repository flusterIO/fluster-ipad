import {
    WebViewContainer,
    MdxEditorPreviewOnly,
    createFlusterStore,
    handleEditorStateParsedContentUpdate,
    MdxEditorGlobalProvider,
    WebviewImplementation,
} from "@fluster/webview_utils";
import React from "react";

const storeData = createFlusterStore();

declare global {
    interface Window {
        handleEditorStateParsedContentUpdate: (data: number[]) => void;
    }
}

window.handleEditorStateParsedContentUpdate =
    handleEditorStateParsedContentUpdate;

function App() {
    return (
        <WebViewContainer>
            <MdxEditorGlobalProvider
                store={storeData.store}
                persistor={storeData.persistor}
            >
                <MdxEditorPreviewOnly
                    implementation={WebviewImplementation.MdxViewer}
                    className={"px-6 pb-12 pt-8"}
                />
            </MdxEditorGlobalProvider>
        </WebViewContainer>
    );
}

export default App;
