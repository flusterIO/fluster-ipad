import React from "react";
import {
    type AnyCrossLanguageBufferEditorAction,
    type AnyCrossLanguageEditorAction,
    BibtexEditor,
    createFlusterStore,
    handleSwiftAction,
    handleSwiftBufferAction,
    MdxEditorGlobalProvider,
    WebViewContainer,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";
import { ByteBuffer } from "flatbuffers";

declare global {
    interface Window {
        handleEditorStateParsedContentUpdate: (data: number[]) => void;
        handleSwiftAction: typeof handleSwiftActionWrapper;
        handleSwiftBufferAction: typeof handleSwiftBufferActionWrapper;
    }
}

const storeData = createFlusterStore();

const handleSwiftActionWrapper = (
    action: AnyCrossLanguageEditorAction,
): void => {
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
            <WebViewContainer contentContainerClasses="h-full">
                <BibtexEditor haveSetInitialValue={false} />
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    );
}

export default App;
