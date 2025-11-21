export interface SwiftEventMap {
    updateEditorValue: string;
}

declare global {
    // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
    interface WindowEventMap {
        "set-swift-editor-content": CustomEvent<string>;
    }
    interface Window {
        setEditorContent: typeof setEditorContent;
    }
}

export function setEditorContent(payload: string) {
    window.dispatchEvent(
        new CustomEvent("set-swift-editor-content", { detail: payload }),
    );
}

export const setWindowBridgeFunctions = () => {
    window.setEditorContent = setEditorContent;
};
