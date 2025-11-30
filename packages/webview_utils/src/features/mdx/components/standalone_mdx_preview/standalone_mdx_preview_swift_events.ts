declare global {
    interface Window {
        setMdxPreviewContent: typeof setMdxPreviewContent;
    }
}

const setMdxPreviewContent = (mdxContent: string): void => {
    window.dispatchEvent(
        new CustomEvent("set-mdx-preview-content", {
            detail: mdxContent,
        }),
    );
};

export const setMdxPreviewWindowMethods = () => {
    window.setMdxPreviewContent = setMdxPreviewContent;
};
