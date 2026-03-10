import React, { useEffect, useState } from "react";
import {
    createFlusterStore,
    LoadingComponent,
    MdxEditorGlobalProvider,
    MdxSerialization,
    NoteDetailSheet,
    NoteDetailWebviewActions,
    NoteDetailWebviewEvents,
    sendToSwift,
    useEventListener,
    WebViewContainer,
    handleSwiftActionWrapper,
    handleSwiftBufferActionWrapper,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/styles.css";
import "./index.css";
import { ByteBuffer } from "flatbuffers";

const storeData = createFlusterStore();

window.handleSwiftAction = handleSwiftActionWrapper(storeData.store);

window.handleSwiftBufferAction = handleSwiftBufferActionWrapper(storeData.store);

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
