export interface SwiftEventMap {
    updateEditorValue: string;
}

declare global {
    // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
    interface WindowEventMap {
        "set-editor-content": CustomEvent<string>;
        "set-editor-keymap": CustomEvent<string>;
        "set-editor-theme": CustomEvent<string>;
        "set-editor-theme-dark": CustomEvent<string>;
        "set-editor-theme-light": CustomEvent<string>;
        "reset-mdx-preview-scroll-position": CustomEvent<null>;
    }
    interface Window {
        setEditorContent: typeof setEditorContent;
        setEditorKeymap: typeof setEditorKeymap;
        setEditorTheme: typeof setEditorTheme;
        setEditorThemeLight: typeof setEditorThemeLight;
        setEditorThemeDark: typeof setEditorThemeDark;
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

export function setEditorTheme(theme: string) {
    window.dispatchEvent(new CustomEvent("set-editor-theme", { detail: theme }));
}

export function setEditorThemeDark(theme: string) {
    window.dispatchEvent(
        new CustomEvent("set-editor-theme-dark", { detail: theme }),
    );
}

export function setEditorThemeLight(theme: string) {
    window.dispatchEvent(
        new CustomEvent("set-editor-theme-light", { detail: theme }),
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
    window.setEditorTheme = setEditorTheme;
    window.setEditorThemeDark = setEditorThemeDark;
    window.setEditorThemeLight = setEditorThemeLight;
    window.resetMdxPreviewScrollPosition = resetMdxPreviewScrollPosition;
};
