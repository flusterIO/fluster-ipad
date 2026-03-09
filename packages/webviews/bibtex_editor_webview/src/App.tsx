import React from "react";
import {
    BibtexEditor,
    createFlusterStore,
    MdxEditorGlobalProvider,
    WebViewContainer,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";

const storeData = createFlusterStore();

function App() {
    return (
        <WebViewContainer contentContainerClasses="h-full">
            <MdxEditorGlobalProvider
                store={storeData.store}
                persistor={storeData.persistor}
            >
                <BibtexEditor haveSetInitialValue={false} />
            </MdxEditorGlobalProvider>
        </WebViewContainer>
    );
}

export default App;
