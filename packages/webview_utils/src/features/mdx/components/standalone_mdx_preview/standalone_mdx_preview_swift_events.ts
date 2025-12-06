import { SplitviewEditorWebviewEvents } from "@/code_gen/typeshare/fluster_core_utilities";

declare global {
    interface Window {
        setMdxPreviewContent: typeof setMdxPreviewContent;
    }
}

const setMdxPreviewContent = (mdxContent: string): void => {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetParsedMdxContent, {
            detail: mdxContent,
        }),
    );
};

export const setMdxPreviewWindowMethods = () => {
    window.setMdxPreviewContent = setMdxPreviewContent;
};
