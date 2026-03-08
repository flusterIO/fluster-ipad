import React from "react";
import {
    ResponsiveSplitViewEditor,
    MdxEditorGlobalProvider,
    WebViewContainer,
    createFlusterStore,
    WebviewEnvironment,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";

const storeData = createFlusterStore(
    process.env.FLUSTER_BUILD_ENV === "ipad"
        ? WebviewEnvironment.IPad
        : WebviewEnvironment.MacOS,
);

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
