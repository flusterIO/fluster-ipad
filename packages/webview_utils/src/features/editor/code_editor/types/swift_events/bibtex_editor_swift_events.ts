import { SerializedString } from "@/code_gen/flat_buffer/shared-webview-data";
import { BibtexEditorWebviewEvents, BibtexEditorWebviewLocalStorageKeys } from "@/code_gen/typeshare/fluster_core_utilities";
import { ByteBuffer } from "flatbuffers";

declare global {
    // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
    interface WindowEventMap {
        [BibtexEditorWebviewEvents.SetBibtexEditorContent]: CustomEvent<SerializedString>;
    }
    interface Window {
        setBibtexEditorContent: typeof setBibtexEditorContent;
        clearBibtexEditorData: typeof clearBibtexEditorData;
    }
}

const clearBibtexEditorData = () => {
    window.localStorage.removeItem(BibtexEditorWebviewLocalStorageKeys.InitialValue);
};

export function setBibtexEditorContent(payload: Uint8Array) {
    const data = Uint8Array.from(payload)
    const buf = new ByteBuffer(data)
    window.dispatchEvent(
        new CustomEvent(BibtexEditorWebviewEvents.SetBibtexEditorContent, { detail: SerializedString.getRootAsSerializedString(buf) }),
    );
}

export const setBibtexEditorWindowBridgeFunctions = () => {
    window.setBibtexEditorContent = setBibtexEditorContent;
    window.clearBibtexEditorData = clearBibtexEditorData;
};
