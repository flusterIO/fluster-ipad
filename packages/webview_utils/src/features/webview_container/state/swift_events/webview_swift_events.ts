import { ScreenDimensions } from "@/state/hooks/use_screen_dimensions";

declare global {
    interface WindowEventMap {
        "set-dark-mode": CustomEvent<boolean>;
        "set-webview-theme": CustomEvent<string>;
        "set-webview-font-size": CustomEvent<string>;
        "set-screen-size": CustomEvent<ScreenDimensions>;
    }
    interface Window {
        setDarkMode: typeof setDarkMode;
        setWebviewTheme: typeof setWebviewTheme;
        setWebViewFontSize: typeof setWebViewFontSize;
        setScreenSize: typeof setScreenSize;
    }
}

export function setDarkMode(darkMode: boolean) {
    window.dispatchEvent(new CustomEvent("set-dark-mode", { detail: darkMode }));
    const h = document.querySelector("html")
    if (h) {
        h.style.backgroundColor = darkMode ? "black" : "white"
    }
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

export const setWebviewWindowBridgeFunctions = () => {
    window.setDarkMode = setDarkMode;
    window.setWebviewTheme = setWebviewTheme;
    window.setWebViewFontSize = setWebViewFontSize;
    window.setScreenSize = setScreenSize;
};
