import { ByteBuffer } from "flatbuffers";
import { handleSwiftAction, handleSwiftBufferAction } from "./container/webview_container_global_state/webview_container_slice";
import { type Store } from "@reduxjs/toolkit";
import { type GlobalAppState } from "./store";
import { type AnyCrossLanguageWebviewAction, type AnyCrossLanguageBufferEditorAction } from "./cross_language_state_types"


declare global {
    interface Window {
        handleEditorStateParsedContentUpdate: (data: number[]) => void;
        handleSwiftAction: ReturnType<typeof handleSwiftActionWrapper>;
        handleSwiftBufferAction: ReturnType<typeof handleSwiftBufferActionWrapper>;
    }
}

export const handleSwiftBufferActionWrapper = (store: Store<GlobalAppState>) => (
    actionKey: AnyCrossLanguageBufferEditorAction["type"],
    payloadBuffer: number[],
): void => {
    const data = Uint8Array.from(payloadBuffer);
    const buf = new ByteBuffer(data);

    store.dispatch(
        handleSwiftBufferAction({
            type: actionKey,
            payload: buf,
        }),
    );
}


export const handleSwiftActionWrapper = (store: Store<GlobalAppState>) => (action: AnyCrossLanguageWebviewAction): void => {
    store.dispatch(handleSwiftAction(action))
}
