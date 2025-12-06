import { BibtexEditorWebviewEvents, BibtexEditorWebviewLocalStorageKeys } from "@/code_gen/typeshare/fluster_core_utilities";

declare global {
    // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
    interface WindowEventMap {
        [BibtexEditorWebviewEvents.SetBibtexEditorContent]: CustomEvent<string>;
    }
    interface Window {
        setBibtexEditorContent: typeof setBibtexEditorContent;
        clearBibtexEditorData: typeof clearBibtexEditorData;
    }
}

const clearBibtexEditorData = () => {
    window.localStorage.removeItem(BibtexEditorWebviewLocalStorageKeys.InitialValue);
};

export function setBibtexEditorContent(payload: string) {
    window.dispatchEvent(
        new CustomEvent(BibtexEditorWebviewEvents.SetBibtexEditorContent, { detail: payload }),
    );
}

export const setBibtexEditorWindowBridgeFunctions = () => {
    window.setBibtexEditorContent = setBibtexEditorContent;
    window.clearBibtexEditorData = clearBibtexEditorData;
};
