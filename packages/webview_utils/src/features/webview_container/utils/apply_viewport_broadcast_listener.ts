import { sendToSwift, SwiftHandler } from "@/utils/bridge/send_to_swift";
import { CaseUpper } from "lucide-react";

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
    sizeElementId: string = "webview-content-wrapper",
): void => {
    const height = (): string =>
        document.getElementById(sizeElementId)!.scrollHeight.toString();
    const handleResize = (): void => {
        console.log("Sending viewport size to swift");
        console.log("sending height: ", height());
        const isLoading = document
            .getElementById("webview-container")
            ?.classList.contains("loading");
        if (!isLoading) {
            sendToSwift(broadcastKey, height());
        }
    };
    window.requestDocumentSize = () => {
        sendToSwift(broadcastKey, height());
    };
    screen.orientation.addEventListener("change", handleResize);
    document
        .getElementById(sizeElementId)
        ?.addEventListener("resize", handleResize);
    window.addEventListener("request-document-size", handleResize);
};
