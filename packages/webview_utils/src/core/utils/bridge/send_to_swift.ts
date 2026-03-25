import { type NoteDetailEvents } from "@/code_gen/typeshare/fluster_core_utilities";
import { type AnyNewReduxAction, type AnyWebviewAction } from "../types/any_window_event";

declare global {
    interface Window {
        webkit: {
            messageHandlers: Record<string, { postMessage: (msg: string | Uint8Array) => void }>;
        };
    }
}

export type SwiftBridgeMessageObject = Record<string, object | string | number>;


export const sendToSwift = (
    handler: AnyWebviewAction | AnyNewReduxAction | NoteDetailEvents,
    msg: SwiftBridgeMessageObject | string = "",
) => {
    if (
        window.webkit &&
        window.webkit.messageHandlers &&
        window.webkit.messageHandlers[handler]
    ) {
        if (handler in window.webkit.messageHandlers) {
            window.webkit.messageHandlers[handler].postMessage(
                typeof msg === "string" ? msg : JSON.stringify(msg),
            );
        }
    } else {
        console.error(`Swift handler not found for ${handler}`)
    }
};
