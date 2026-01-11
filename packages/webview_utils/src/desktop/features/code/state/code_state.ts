import { BundledFlusterTheme } from "../data/bundled_themes";

export interface JupyterConfigState {
    port: number;
    defaultKernelName: string;
    token: string;
}

export interface CodeState {
    keymap: "vim" | "standard";
    defaultLanguage: string;
    theme: {
        dark: BundledFlusterTheme;
        light: BundledFlusterTheme;
    };
    /** The debounce timeout in seconds. */
    previewDebounce: number;
    jupyter: JupyterConfigState;
}
