import React from "react";
import {
    WebViewContainer,
    createFlusterStore,
    MdxEditorGlobalProvider,
    handleSwiftActionWrapper,
    handleSwiftBufferActionWrapper,
} from "@fluster/webview_utils";
import { EditorMain } from "./editor/editor_main"
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
                <EditorMain />
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    );
}

export default App;
