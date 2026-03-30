import {
    CodeEditor,
    createFlusterStore,
    handleSwiftActionWrapper,
    handleSwiftBufferActionWrapper,
    MdxEditorGlobalProvider,
    WebViewContainer,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import React from "react";

const storeData = createFlusterStore();

window.handleSwiftAction = handleSwiftActionWrapper(storeData.store);

window.handleSwiftBufferAction = handleSwiftBufferActionWrapper(
    storeData.store,
);

function App() {
    return (
        <MdxEditorGlobalProvider
            store={storeData.store}
            persistor={storeData.persistor}
        >
            <WebViewContainer>
                <CodeEditor />
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    );
}

export default App;
