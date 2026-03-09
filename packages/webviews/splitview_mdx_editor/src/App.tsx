import React from "react";
import {
    ResponsiveSplitViewEditor,
    MdxEditorGlobalProvider,
    WebViewContainer,
    createFlusterStore,
    handleEditorStateParsedContentUpdate,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";

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
        <WebViewContainer contentContainerClasses="h-full">
            <MdxEditorGlobalProvider
                store={storeData.store}
                persistor={storeData.persistor}
            >
                <ResponsiveSplitViewEditor />
            </MdxEditorGlobalProvider>
        </WebViewContainer>
    );
}

export default App;
