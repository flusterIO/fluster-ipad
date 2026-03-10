import React from "react";
import {
    DictionaryPage,
    useDictionaryData,
    WebViewContainer,
    LoadingComponent,
    createFlusterStore,
    MdxEditorGlobalProvider,
    handleSwiftActionWrapper,
    handleSwiftBufferActionWrapper,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";

const storeData = createFlusterStore();

window.handleSwiftAction = handleSwiftActionWrapper(storeData.store);

window.handleSwiftBufferAction = handleSwiftBufferActionWrapper(storeData.store);

function App() {
    const data = useDictionaryData();
    return (
        <MdxEditorGlobalProvider
            store={storeData.store}
            persistor={storeData.persistor}
        >
            <WebViewContainer contentContainerClasses="h-full">
                {data ? (
                    <DictionaryPage data={data} />
                ) : (
                    <div className="w-full h-full flex flex-col justfy-center items-center">
                        <LoadingComponent />
                    </div>
                )}
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    );
}

export default App;
