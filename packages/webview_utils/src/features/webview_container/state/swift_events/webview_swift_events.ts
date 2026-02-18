import { ScreenDimensions } from "@/state/hooks/use_screen_dimensions";


// TODO: Migrate these all to enums
declare global {
    interface WindowEventMap {
        "set-dark-mode": CustomEvent<boolean>;
        "set-webview-theme": CustomEvent<string>;
        "set-webview-font-size": CustomEvent<string>;
        "set-screen-size": CustomEvent<ScreenDimensions>;
        "lock-editor-scroll-to-preview": CustomEvent<boolean>;
    }
    interface Window {
        setDarkMode: typeof setDarkMode;
        setWebviewTheme: typeof setWebviewTheme;
        setWebViewFontSize: typeof setWebViewFontSize;
        setScreenSize: typeof setScreenSize;
        setLockEditorScrollToPreview: typeof setLockWebviewScrollToPreview
    }
}

export function setDarkMode(darkMode: boolean) {
    window.dispatchEvent(new CustomEvent("set-dark-mode", { detail: darkMode }));
    window.localStorage.setItem("dark-mode", darkMode ? "true" : "false")
}

export function setWebviewTheme(webviewTheme: string) {
    window.dispatchEvent(
        new CustomEvent("set-webview-theme", { detail: webviewTheme }),
    );
}

export function setWebViewFontSize(cssClass: string) {
    window.localStorage.setItem("webview-font-class", cssClass);
    window.dispatchEvent(
        new CustomEvent("set-webview-font-size", { detail: cssClass }),
    );
}

export function setScreenSize(width: number, height: number) {
    window.dispatchEvent(
        new CustomEvent("set-screen-size", {
            detail: {
                width,
                height,
            } satisfies ScreenDimensions,
        }),
    );
}


const setLockWebviewScrollToPreview = (lock: boolean): void => {
    window.dispatchEvent(
        new CustomEvent("lock-editor-scroll-to-preview", {
            detail: lock
        })
    )
}

export const setWebviewWindowBridgeFunctions = () => {
    window.setDarkMode = setDarkMode;
    window.setWebviewTheme = setWebviewTheme;
    window.setWebViewFontSize = setWebViewFontSize;
    window.setScreenSize = setScreenSize;
    window.setLockEditorScrollToPreview = setLockWebviewScrollToPreview;
};
