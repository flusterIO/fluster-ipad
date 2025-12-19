import React, { useEffect, useState } from "react";
import {
    LoadingComponent,
    MdxSerialization,
    NoteDetailSheet,
    NoteDetailWebviewActions,
    NoteDetailWebviewEvents,
    sendToSwift,
    useEventListener,
    WebViewContainer,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/webview_utils.css";
import "./index.css";
import { ByteBuffer } from "flatbuffers";

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
        <WebViewContainer
            style={{
                backgroundColor: "hsl(var(--background))",
            }}
            contentContainerClasses="h-full"
        >
            {data ? (
                <NoteDetailSheet data={data} />
            ) : (
                <div className="w-full h-full flex flex-col justify-center items-center loading-show">
                    <LoadingComponent />
                </div>
            )}
        </WebViewContainer>
    );
}

export default App;
