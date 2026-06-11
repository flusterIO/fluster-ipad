import React from "react";
import {
    WebViewContainer,
    createFlusterStore,
    MdxEditorGlobalProvider,
    handleSwiftActionWrapper,
    handleSwiftBufferActionWrapper,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";
import { DictionaryPage } from "./dictionary_page";
import initCdrmWasm from "@conundrum/wasm";

const storeData = createFlusterStore();

window.handleSwiftAction = handleSwiftActionWrapper(storeData.store);

window.handleSwiftBufferAction = handleSwiftBufferActionWrapper(
    storeData.store,
);

initCdrmWasm().catch((err: unknown) => {
    console.log("err: ", err);
});

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
