declare global {
    // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
    interface WindowEventMap {
        "set-bibtex-editor-content": CustomEvent<string>;
    }
    interface Window {
        setBibtexEditorContent: typeof setBibtexEditorContent;
        clearBibtexEditorData: typeof clearBibtexEditorData;
    }
}

const clearBibtexEditorData = () => {
    console.log(`Clearing bibtex editor data...`);
    window.localStorage.removeItem("bibtex-editor-initial-value");
};

export function setBibtexEditorContent(payload: string) {
    window.dispatchEvent(
        new CustomEvent("set-bibtex-editor-content", { detail: payload }),
    );
}

export const setBibtexEditorWindowBridgeFunctions = () => {
    window.setBibtexEditorContent = setBibtexEditorContent;
    window.clearBibtexEditorData = clearBibtexEditorData;
};
