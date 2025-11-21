declare global {
    interface WindowEventMap {
        "set-dark-mode": CustomEvent<boolean>;
        "set-webview-theme": CustomEvent<string>;
    }
    interface Window {
        setDarkMode: typeof setDarkMode;
        setWebviewTheme: typeof setWebviewTheme;
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

export const setWebviewWindowBridgeFunctions = () => {
    window.setDarkMode = setDarkMode;
    window.setWebviewTheme = setWebviewTheme;
};
