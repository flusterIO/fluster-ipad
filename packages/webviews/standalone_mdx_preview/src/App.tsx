import {
    WebViewContainer,
    MdxEditorPreviewOnly,
    createFlusterStore,
    MdxEditorGlobalProvider,
    WebviewImplementation,
    handleSwiftActionWrapper,
    handleSwiftBufferActionWrapper,
} from "@fluster/webview_utils";
import React from "react";

const storeData = createFlusterStore();

window.handleSwiftAction = handleSwiftActionWrapper(storeData.store);

window.handleSwiftBufferAction = handleSwiftBufferActionWrapper(storeData.store);

function App() {
    return (
        <MdxEditorGlobalProvider
            store={storeData.store}
            persistor={storeData.persistor}
        >
            <WebViewContainer>
                <MdxEditorPreviewOnly
                    implementation={WebviewImplementation.MdxViewer}
                    className={"px-6 pb-12 pt-8"}
                />
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    );
}

export default App;
