import { ByteBuffer } from "flatbuffers";
import { handleSwiftAction, handleSwiftBufferAction } from "./container/webview_container_global_state/webview_container_slice";
import { type Store } from "@reduxjs/toolkit";
import { type MdxEditorAppState } from "./store";
import {type AnyCrossLanguageBufferEditorAction, type AnyCrossLanguageEditorAction} from "./cross_language_state_types"


declare global {
    interface Window {
        handleEditorStateParsedContentUpdate: (data: number[]) => void;
        handleSwiftAction: ReturnType<typeof handleSwiftActionWrapper>;
        handleSwiftBufferAction: ReturnType<typeof handleSwiftBufferActionWrapper>;
    }
}

export const handleSwiftBufferActionWrapper = (store: Store<MdxEditorAppState>) => (
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


export const handleSwiftActionWrapper = (store: Store<MdxEditorAppState>) => (action: AnyCrossLanguageEditorAction): void => {
    store.dispatch(handleSwiftAction(action))
}
