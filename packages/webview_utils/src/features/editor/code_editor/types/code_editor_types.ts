export enum CodeEditorBaseKeymap {
    default,
    vscode,
}

export enum CodeEditorTheme {
    // Light
    materialLight,
    solarizedLight,
    solarizedDark,
    githubLight,
    aura,
    tokyoNightDay,
    xcodeLight,
    // Dark
    dracula,
    tokyoNight,
    materialDark,
    tokyoNightStorm,
    githubDark,
    xcodeDark,
}

export const stringToCodeEditorTheme = (val: string): CodeEditorTheme => {
    if (val === "materialLight") {
        return CodeEditorTheme.materialLight;
    }
    if (val === "solarizedLight") {
        return CodeEditorTheme.solarizedLight;
    }
    if (val === "solarizedDark") {
        return CodeEditorTheme.solarizedDark;
    }
    if (val === "githubLight") {
        return CodeEditorTheme.githubLight;
    }
    if (val === "aura") {
        return CodeEditorTheme.aura;
    }
    if (val === "tokyoNightDay") {
        return CodeEditorTheme.tokyoNightDay;
    }
    if (val === "xcodeLight") {
        return CodeEditorTheme.xcodeLight;
    }
    if (val === "dracula") {
        return CodeEditorTheme.dracula;
    }
    if (val === "tokyoNight") {
        return CodeEditorTheme.tokyoNight;
    }
    if (val === "materialDark") {
        return CodeEditorTheme.materialDark;
    }
    if (val === "tokyoNightStorm") {
        return CodeEditorTheme.tokyoNightStorm;
    }
    if (val === "githubDark") {
        return CodeEditorTheme.githubDark;
    }
    if (val === "xcodeDark") {
        return CodeEditorTheme.xcodeDark;
    }
    return CodeEditorTheme.dracula;
};

export enum CodeEditorLanguage {
    markdown,
    bibtex,
}

declare global {
    interface WindowEventMap {
        "set-editor-theme": CustomEvent<string>;
        "set-editor-keymap": CustomEvent<string>;
        "set-initial-editor-content": CustomEvent<string>;
    }
}
