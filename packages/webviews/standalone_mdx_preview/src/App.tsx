import {
    WebViewContainer,
    MdxEditorPreviewOnly,
    createFlusterStore,
    MdxEditorGlobalProvider,
    WebviewImplementation,
    type AnyCrossLanguageEditorAction,
    handleSwiftAction,
    type AnyCrossLanguageBufferEditorAction,
    handleSwiftBufferAction,
} from "@fluster/webview_utils";
import React from "react";
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
    return (
        <MdxEditorGlobalProvider
            store={storeData.store}
            persistor={storeData.persistor}
        >
            <WebViewContainer>
                <MdxEditorPreviewOnly
                    implementation={WebviewImplementation.MdxViewer}
                    className={"px-6 pb-12 pt-8"}
                />
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    );
}

export default App;
