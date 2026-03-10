import React from "react";
import {
    BibtexEditor,
    createFlusterStore,
    MdxEditorGlobalProvider,
    WebViewContainer,
    handleSwiftBufferActionWrapper,
    handleSwiftActionWrapper
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";


const storeData = createFlusterStore();

window.handleSwiftAction = handleSwiftActionWrapper(storeData.store);

window.handleSwiftBufferAction = handleSwiftBufferActionWrapper(storeData.store);

function App() {
    return (
        <MdxEditorGlobalProvider
            store={storeData.store}
            persistor={storeData.persistor}
        >
            <WebViewContainer contentContainerClasses="h-full">
                <BibtexEditor haveSetInitialValue={false} />
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    );
}

export default App;
