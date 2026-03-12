import React from "react";
import {
    createFlusterStore,
    MdxEditorGlobalProvider,
    NoteDetailSheet,
    WebViewContainer,
    handleSwiftActionWrapper,
    handleSwiftBufferActionWrapper,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";

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
            <WebViewContainer contentContainerClasses="h-full">
                <NoteDetailSheet />
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    );
}

export default App;
