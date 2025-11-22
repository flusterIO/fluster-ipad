declare global {
    interface WindowEventMap {
        "set-dark-mode": CustomEvent<boolean>;
        "set-webview-theme": CustomEvent<string>;
        "set-webview-font-size": CustomEvent<string>;
    }
    interface Window {
        setDarkMode: typeof setDarkMode;
        setWebviewTheme: typeof setWebviewTheme;
        setWebViewFontSize: typeof setWebViewFontSize;
    }
}

export function setDarkMode(darkMode: boolean) {
    window.dispatchEvent(new CustomEvent("set-dark-mode", { detail: darkMode }));
}

export function setWebviewTheme(webviewTheme: string) {
    window.dispatchEvent(
        new CustomEvent("set-webview-theme", { detail: webviewTheme }),
    );
}

export function setWebViewFontSize(cssClass: string) {
    window.dispatchEvent(
        new CustomEvent("set-webview-font-size", { detail: cssClass }),
    );
}

export const setWebviewWindowBridgeFunctions = () => {
    window.setDarkMode = setDarkMode;
    window.setWebviewTheme = setWebviewTheme;
    window.setWebViewFontSize = setWebViewFontSize;
};
