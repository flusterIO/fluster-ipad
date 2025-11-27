export enum SwiftHandler {
    editorUpdate = "editor-update",
    requestInitialData = "request-initial-data",
    bibtexRequestInitialData = "bibtex-request-initial-data",
    bibtexEditorUpdate = "bibtex-editor-update",
}

declare global {
    interface Window {
        webkit: {
            messageHandlers: Record<string, { postMessage: (msg: string) => void }>;
        };
    }
}

export interface SwiftBridgeMessageObject {
    [key: string]: object | string | number;
}

export const sendToSwift = (
    handler: SwiftHandler,
    msg: SwiftBridgeMessageObject | string,
) => {
    if (
        window.webkit &&
        window.webkit.messageHandlers &&
        window.webkit.messageHandlers[handler]
    ) {
        window.webkit.messageHandlers[handler].postMessage(
            typeof msg === "string" ? msg : JSON.stringify(msg),
        );
    }
};
