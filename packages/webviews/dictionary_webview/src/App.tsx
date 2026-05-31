import React from "react";
import {
    WebViewContainer,
    createFlusterStore,
    MdxEditorGlobalProvider,
    handleSwiftActionWrapper,
    handleSwiftBufferActionWrapper,
    DictionaryWebviewIds,
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
                <div className="w-full max-w-[1080px] px-8 ml-auto mr-auto">
                    <h1 className="mb-8 text-xl @md/cdrm:text-2xl @lg:/cdrm:text-3xl hide-desktop">
                        Dictionary
                    </h1>
                    <div
                        id={DictionaryWebviewIds.DictionaryDataContainer}
                        className="w-full h-full flex flex-col justify-center items-center p-4"
                    >
                        <h3 className="text-foreground/80 w-fit text-center">
                            No Dictionary Entries Found
                        </h3>
                    </div>
                </div>
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    );
}

export default App;
