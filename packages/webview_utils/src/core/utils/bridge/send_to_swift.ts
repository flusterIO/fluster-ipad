import { AnyWebviewAction } from "../types/any_window_event";

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
    handler: AnyWebviewAction,
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
    } else {
        console.error(`Swift handler not found for ${handler}`)
    }
};
