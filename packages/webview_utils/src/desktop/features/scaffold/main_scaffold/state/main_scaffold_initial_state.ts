import { supportedTheme } from "src/desktop/core/core_types/supported_themes";


export enum ThemeMode {
    light,
    dark,
    system,
}

export interface ScaffoldState {
    themeMode: ThemeMode;
    theme: (typeof supportedTheme)[number];
}

export const initialScaffoldState: ScaffoldState = {
    themeMode: ThemeMode.system,
    theme: "fluster",
};
