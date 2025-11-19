export interface SetWebviewColorSchemeProps {
    darkMode: boolean;
}

declare global {
    interface WindowEventMap {
        "set-webview-color-scheme": CustomEvent<SetWebviewColorSchemeProps>;
    }
}
