// This file was auto-generated during the build process. Any changes made here will be lost.

export type EditorDarkThemeValue =  | "Andromeeda" | "Aurora X" | "Ayu Dark" | "Catppuccin Frappé" | "Catppuccin Macchiato" | "Catppuccin Mocha" | "Dark Plus" | "Dracula Theme" | "Dracula Theme Soft" | "Everforest Dark" | "GitHub Dark" | "GitHub Dark Default" | "GitHub Dark Dimmed" | "GitHub Dark High Contrast" | "Gruvbox Dark Hard" | "Gruvbox Dark Medium" | "Gruvbox Dark Soft" | "Houston" | "Kanagawa Dragon" | "Kanagawa Wave" | "LaserWave" | "Material Theme" | "Material Theme Darker" | "Material Theme Ocean" | "Material Theme Palenight" | "Min Dark" | "Monokai" | "Night Owl" | "Nord" | "One Dark Pro" | "Plastic" | "Poimandres" | "Red" | "Rosé Pine" | "Rosé Pine Moon" | "Slack Dark" | "Solarized Dark" | "Synthwave '84" | "Tokyo Night" | "Vesper" | "Vitesse Black" | "Vitesse Dark"

export type EditorLightThemeValue =  | "Catppuccin Latte" | "Everforest Light" | "GitHub Light" | "GitHub Light Default" | "GitHub Light High Contrast" | "Gruvbox Light Hard" | "Gruvbox Light Medium" | "Gruvbox Light Soft" | "Kanagawa Lotus" | "Light Plus" | "Material Theme Lighter" | "Min Light" | "One Light" | "Rosé Pine Dawn" | "Slack Ochin" | "Snazzy Light" | "Solarized Light" | "Vitesse Light"

export const getEditorLightThemeValues = () => {
     return [
        "Catppuccin Latte",
        "Everforest Light",
        "GitHub Light",
        "GitHub Light Default",
        "GitHub Light High Contrast",
        "Gruvbox Light Hard",
        "Gruvbox Light Medium",
        "Gruvbox Light Soft",
        "Kanagawa Lotus",
        "Light Plus",
        "Material Theme Lighter",
        "Min Light",
        "One Light",
        "Rosé Pine Dawn",
        "Slack Ochin",
        "Snazzy Light",
        "Solarized Light",
        "Vitesse Light",
    ] as EditorLightThemeValue[];
}


export const getEditorDarkThemeValues = () => {
     return [
        "Andromeeda",
        "Aurora X",
        "Ayu Dark",
        "Catppuccin Frappé",
        "Catppuccin Macchiato",
        "Catppuccin Mocha",
        "Dark Plus",
        "Dracula Theme",
        "Dracula Theme Soft",
        "Everforest Dark",
        "GitHub Dark",
        "GitHub Dark Default",
        "GitHub Dark Dimmed",
        "GitHub Dark High Contrast",
        "Gruvbox Dark Hard",
        "Gruvbox Dark Medium",
        "Gruvbox Dark Soft",
        "Houston",
        "Kanagawa Dragon",
        "Kanagawa Wave",
        "LaserWave",
        "Material Theme",
        "Material Theme Darker",
        "Material Theme Ocean",
        "Material Theme Palenight",
        "Min Dark",
        "Monokai",
        "Night Owl",
        "Nord",
        "One Dark Pro",
        "Plastic",
        "Poimandres",
        "Red",
        "Rosé Pine",
        "Rosé Pine Moon",
        "Slack Dark",
        "Solarized Dark",
        "Synthwave '84",
        "Tokyo Night",
        "Vesper",
        "Vitesse Black",
        "Vitesse Dark",
    ] as EditorDarkThemeValue[];
}

export const getAllEditorThemes = () => {
    return [...getEditorDarkThemeValues(), ...getEditorLightThemeValues()]
}

export type BundledThemeValue = EditorLightThemeValue | EditorDarkThemeValue

