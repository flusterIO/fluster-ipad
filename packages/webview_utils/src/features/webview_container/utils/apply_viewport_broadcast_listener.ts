import { sendToSwift, SwiftHandler } from "@/utils/bridge/send_to_swift";
import { getParentContentWrapper } from "../presentation/webview_container";

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
    broadcastKey: SwiftHandler,
    /// The optional id of the element that wraps the documents content. This is required to avoidd expanding to meet the viewport when height: 100vh
    sizeElement: () => HTMLElement | null = getParentContentWrapper,
): void => {
    const height = (): string | undefined =>
        sizeElement()?.scrollHeight.toString();
    const handleResize = (): void => {
        console.log("Sending viewport size to swift");
        const isLoading = document
            .getElementById("webview-container")
            ?.classList.contains("loading");
        if (!isLoading) {
            const x = height();
            console.log("height in handleResize: ", x);
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
