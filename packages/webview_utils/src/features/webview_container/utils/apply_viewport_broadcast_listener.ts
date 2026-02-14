import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { getParentContentWrapper } from "../presentation/webview_container";
import { AnyWebviewAction } from "@/utils/types/any_window_event";

/// Applies a listener that will broadcast the viewport dimensions to swift.
declare global {
    interface Window {
        requestDocumentSize: () => void;
    }
    // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
    interface WindowEventMap {
        "request-document-size": CustomEvent<null>;
    }
}

export const applyViewportBroadcastListener = (
    broadcastKey: AnyWebviewAction,
    /// The optional id of the element that wraps the documents content. This is required to avoidd expanding to meet the viewport when height: 100vh
    sizeElement: () => HTMLElement | null = getParentContentWrapper,
): void => {
    const height = (): string | undefined =>
        sizeElement()?.scrollHeight.toString();
    const handleResize = (): void => {
        const isLoading = document
            .body?.classList.contains("loading");
        if (!isLoading) {
            const x = height();
            if (x) {
                sendToSwift(broadcastKey, x);
            }
        }
    };
    window.requestDocumentSize = () => {
        const h = height();
        console.log("height in requestDocumentSize: ", h);
        if (h) {
            sendToSwift(broadcastKey, h);
        }
    };
    screen.orientation.addEventListener("change", handleResize);
    sizeElement()?.addEventListener("resize", handleResize);
    window.addEventListener("request-document-size", handleResize);
};
