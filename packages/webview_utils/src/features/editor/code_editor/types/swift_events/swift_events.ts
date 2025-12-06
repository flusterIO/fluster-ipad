import { SplitviewEditorWebviewEvents } from "@/code_gen/typeshare/fluster_core_utilities";

export interface SwiftEventMap {
    updateEditorValue: string;
}

declare global {
    interface WindowEventMap {
        [SplitviewEditorWebviewEvents.SetSplitviewEditorContent]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetParsedMdxContent]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetEditorKeymap]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetCodeTheme]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetCodeThemeLight]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.SetCodeThemeDark]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.ResetPreviewScrollPosition]: CustomEvent<null>;
    }
    interface Window {
        setEditorContent: typeof setEditorContent;
        setParsedEditorContent: typeof setParsedEditorContent;
        setEditorKeymap: typeof setEditorKeymap;
        setCodeSyntaxTheme: typeof setCodeTheme;
        setCodeSyntaxThemeLight: typeof setCodeThemeLight;
        setCodeSyntaxThemeDark: typeof setCodeThemeDark;
        resetMdxPreviewScrollPosition: typeof resetMdxPreviewScrollPosition;
    }
}

export function setEditorContent(payload: string) {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetSplitviewEditorContent, { detail: payload }),
    );
}

export function setParsedEditorContent(payload: string) {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetParsedMdxContent, { detail: payload }),
    );
}

export function setEditorKeymap(keymap: string) {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetEditorKeymap, { detail: keymap }),
    );
}

export function setCodeTheme(theme: string) {
    window.dispatchEvent(new CustomEvent(SplitviewEditorWebviewEvents.SetCodeTheme, { detail: theme }));
}

export function setCodeThemeDark(theme: string) {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetCodeThemeDark, { detail: theme }),
    );
}

export function setCodeThemeLight(theme: string) {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.SetCodeThemeLight, { detail: theme }),
    );
}

const resetMdxPreviewScrollPosition = (): void => {
    window.dispatchEvent(
        new CustomEvent(SplitviewEditorWebviewEvents.ResetPreviewScrollPosition, {
            detail: null,
        }),
    );
};

export const setWindowBridgeFunctions = () => {
    window.setEditorContent = setEditorContent;
    window.setEditorKeymap = setEditorKeymap;
    window.setParsedEditorContent = setParsedEditorContent;
    window.setCodeSyntaxTheme = setCodeTheme;
    window.setCodeSyntaxThemeDark = setCodeThemeDark;
    window.setCodeSyntaxThemeLight = setCodeThemeLight;
    window.resetMdxPreviewScrollPosition = resetMdxPreviewScrollPosition;
};
