import React from "react";
import {
    WebViewContainer,
    createFlusterStore,
    MdxEditorGlobalProvider,
    handleSwiftActionWrapper,
    handleSwiftBufferActionWrapper,
    DictionaryPage,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";
import { initializeConundrumWeb } from "@conundrum/ts";

initializeConundrumWeb();

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
                <DictionaryPage />
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    );
}

export default App;
