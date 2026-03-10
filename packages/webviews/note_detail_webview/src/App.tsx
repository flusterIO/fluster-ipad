import React, { useEffect, useState } from "react";
import {
    type AnyCrossLanguageBufferEditorAction,
    type AnyCrossLanguageEditorAction,
    createFlusterStore,
    handleSwiftAction,
    handleSwiftBufferAction,
    LoadingComponent,
    MdxEditorGlobalProvider,
    MdxSerialization,
    NoteDetailSheet,
    NoteDetailWebviewActions,
    NoteDetailWebviewEvents,
    sendToSwift,
    useEventListener,
    WebViewContainer,
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
    const [data, setData] =
        useState<MdxSerialization.NoteDetails.NoteDetailDataBuffer | null>(null);
    useEventListener(NoteDetailWebviewEvents.SetNoteDetails, (e) => {
        try {
            const bytes = new Uint8Array(e.detail);
            const buf = new ByteBuffer(bytes);
            const noteDetails =
                MdxSerialization.NoteDetails.NoteDetailDataBuffer.getRootAsNoteDetailDataBuffer(
                    buf,
                );
            setData(noteDetails);
        } catch (err) {
            console.log("NoteDetails serialization error: ", err);
        }
    });

    useEffect(() => {
        if (!data) {
            sendToSwift(NoteDetailWebviewActions.RequestNoteDetailData);
        }
    }, [data]);

    return (
        <MdxEditorGlobalProvider
            store={storeData.store}
            persistor={storeData.persistor}
        >
            <WebViewContainer contentContainerClasses="h-full">
                {data ? (
                    <NoteDetailSheet data={data} />
                ) : (
                    <div className="w-full h-full flex flex-col justify-center items-center loading-show">
                        <LoadingComponent />
                    </div>
                )}
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    );
}

export default App;
