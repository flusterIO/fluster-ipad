import { setWebviewWindowBridgeFunctions } from "#/webview_container/state/swift_events/webview_swift_events";

export interface SwiftEventMap {
    updateEditorValue: string;
}

declare global {
    // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
    interface WindowEventMap {
        "set-editor-content": CustomEvent<string>;
        "set-editor-keymap": CustomEvent<string>;
        "set-editor-theme": CustomEvent<string>;
    }
    interface Window {
        setEditorContent: typeof setEditorContent;
        setEditorKeymap: typeof setEditorKeymap;
        setEditorTheme: typeof setEditorTheme;
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

export const setWindowBridgeFunctions = () => {
    window.setEditorContent = setEditorContent;
    window.setEditorKeymap = setEditorKeymap;
    window.setEditorTheme = setEditorTheme;
};
