export enum SwiftHandler {
    editorUpdate = "editor-update",
    requestParsedMdxContent = "request-parsed-mdx-content",
    setSplitviewEditorLandscapeView = "is-landscape-view",
    setPreviewViewportSize = "set-preview-viewport-height",
    setEditorViewportSize = "set-editor-viewport-height",
    requestInitialEditorData = "request-initial-editor-data",
    requestInitialPreviewData = "request-initial-preview-data",
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
        if (handler in window.webkit.messageHandlers) {
            window.webkit.messageHandlers?.[handler].postMessage(
                typeof msg === "string" ? msg : JSON.stringify(msg),
            );
        }
    }
};
