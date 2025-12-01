export interface SwiftEventMap {
    updateEditorValue: string;
}

declare global {
    interface WindowEventMap {
        "set-editor-content": CustomEvent<string>;
        "set-editor-keymap": CustomEvent<string>;
        "set-code-theme": CustomEvent<string>;
        "set-code-theme-dark": CustomEvent<string>;
        "set-code-theme-light": CustomEvent<string>;
        "reset-mdx-preview-scroll-position": CustomEvent<null>;
    }
    interface Window {
        setEditorContent: typeof setEditorContent;
        setEditorKeymap: typeof setEditorKeymap;
        setCodeSyntaxTheme: typeof setCodeTheme;
        setCodeSyntaxThemeLight: typeof setCodeThemeLight;
        setCodeSyntaxThemeDark: typeof setCodeThemeDark;
        resetMdxPreviewScrollPosition: typeof resetMdxPreviewScrollPosition;
    }
}

export function setEditorContent(payload: string) {
    window.dispatchEvent(
        new CustomEvent("set-editor-content", { detail: payload }),
    );
}

export function setEditorKeymap(keymap: string) {
    window.dispatchEvent(
        new CustomEvent("set-editor-keymap", { detail: keymap }),
    );
}

export function setCodeTheme(theme: string) {
    window.dispatchEvent(new CustomEvent("set-code-theme", { detail: theme }));
}

export function setCodeThemeDark(theme: string) {
    window.dispatchEvent(
        new CustomEvent("set-code-theme-dark", { detail: theme }),
    );
}

export function setCodeThemeLight(theme: string) {
    window.dispatchEvent(
        new CustomEvent("set-code-theme-light", { detail: theme }),
    );
}

const resetMdxPreviewScrollPosition = (): void => {
    window.dispatchEvent(
        new CustomEvent("reset-mdx-preview-scroll-position", {
            detail: null,
        }),
    );
};

export const setWindowBridgeFunctions = () => {
    window.setEditorContent = setEditorContent;
    window.setEditorKeymap = setEditorKeymap;
    window.setCodeSyntaxTheme = setCodeTheme;
    window.setCodeSyntaxThemeDark = setCodeThemeDark;
    window.setCodeSyntaxThemeLight = setCodeThemeLight;
    window.resetMdxPreviewScrollPosition = resetMdxPreviewScrollPosition;
};
