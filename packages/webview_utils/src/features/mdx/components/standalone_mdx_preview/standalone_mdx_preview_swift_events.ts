import { SerializedString } from "@/code_gen/flat_buffer/shared-webview-data";
import { SplitviewEditorWebviewEvents } from "@/code_gen/typeshare/fluster_core_utilities";
import { ByteBuffer } from "flatbuffers";

declare global {
    interface Window {
        setMdxPreviewContent: typeof setMdxPreviewContent;
    }
}

const setMdxPreviewContent = (mdxContent: Uint8Array): void => {
    const data = Uint8Array.from(mdxContent)
    const buf = new ByteBuffer(data)

    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetParsedMdxContentString, {
            detail: SerializedString.getRootAsSerializedString(buf),
        }),
    );
};

export const setMdxPreviewWindowMethods = () => {
    window.setMdxPreviewContent = setMdxPreviewContent;
};
