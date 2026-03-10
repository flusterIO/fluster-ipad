import React from "react";
import {
    DictionaryPage,
    useDictionaryData,
    setMdxBridgeFunctions,
    WebViewContainer,
    LoadingComponent,
    type AnyCrossLanguageBufferEditorAction,
    type AnyCrossLanguageEditorAction,
    createFlusterStore,
    handleSwiftAction,
    handleSwiftBufferAction,
    MdxEditorGlobalProvider,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";
import { ByteBuffer } from "flatbuffers";

const storeData = createFlusterStore();

declare global {
    interface Window {
        handleEditorStateParsedContentUpdate: (data: number[]) => void;
        handleSwiftAction: typeof handleSwiftActionWrapper;
        handleSwiftBufferAction: typeof handleSwiftBufferActionWrapper;
    }
}

const handleSwiftActionWrapper = (actionString: string): void => {
    const action = JSON.parse(actionString) as AnyCrossLanguageEditorAction;
    storeData.store.dispatch(handleSwiftAction(action));
};

window.handleSwiftAction = handleSwiftActionWrapper;

const handleSwiftBufferActionWrapper = (
    actionKey: AnyCrossLanguageBufferEditorAction["type"],
    payloadBuffer: number[],
): void => {
    const data = Uint8Array.from(payloadBuffer);
    const buf = new ByteBuffer(data);

    storeData.store.dispatch(
        handleSwiftBufferAction({
            type: actionKey,
            payload: buf,
        }),
    );
};

window.handleSwiftBufferAction = handleSwiftBufferActionWrapper;

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
