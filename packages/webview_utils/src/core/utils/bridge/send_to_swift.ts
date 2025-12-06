import { NoteDetailWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { AnyWebviewAction } from "../types/any_window_event";

export enum SwiftHandler {
    editorUpdate = "editor-update",
    setSplitviewEditorLandscapeView = "is-landscape-view",
    setPreviewViewportSize = "set-preview-viewport-height",
    setEditorViewportSize = "set-editor-viewport-height",
    bibtexRequestInitialData = "bibtex-request-initial-data",
    bibtexEditorUpdate = "bibtex-editor-update",
    // -- Requesting data from swift --
    requestParsedMdxContent = "request-parsed-mdx-content",
    requestInitialEditorData = "request-initial-editor-data",
    requestInitialPreviewData = "request-initial-preview-data",
    requestNoteDetailData = "request-note-details",
    // -- user actions --
    handleTagClick = "tag-click-event",
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


// TODO: Get rid of this 'swifthandler' all together now that enums are being generated cross-language.
export const sendToSwift = (
    handler: SwiftHandler | AnyWebviewAction,
    msg: SwiftBridgeMessageObject | string = "",
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
