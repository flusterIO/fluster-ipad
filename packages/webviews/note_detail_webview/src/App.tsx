import React from "react";
import {
    LoadingComponent,
    NoteDetailWebviewEvents,
    useEventListener,
    WebViewContainer,
} from "@fluster/webview_utils";
import "../../../webview_utils/dist/webview_utils.css";
import "./index.css";

function App() {
    useEventListener(NoteDetailWebviewEvents.SetNoteDetails, (e) => {
        try {
            console.log(`Have updated note details`);
            /* const bytes = new Uint8Array(e.detail); */
            /* const buf = new ByteBuffer(bytes); */
            /* const noteDetails = */
            /*     MdxSerialization.NoteDetails.NoteDetailDataBuffer.getRootAsNoteDetailDataBuffer( */
            /*         buf, */
            /*     ); */
            /* console.log("noteDetails: ", noteDetails) */
            /* setData(noteDetails) */
        } catch (err) {
            console.log("NoteDetails serialization error: ", err);
        }
    });
    return (
        <WebViewContainer
            style={{
                backgroundColor: "hsl(var(--background))",
            }}
            contentContainerClasses="h-full"
        >
            <div className="w-full h-full flex flex-col justify-center items-center loading-show">
                <LoadingComponent />
            </div>
        </WebViewContainer>
    );
}

export default App;
