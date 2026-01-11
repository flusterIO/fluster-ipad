import { ReactNode } from "react";
import { CommandPaletteAnyEntry } from "../command_palette_any_entry";
import { GeneralCommandPaletteItem } from "../general_command_palette_item";
import { CommandPaletteCategory } from "../models/command_palette_category";
import store from "src/desktop/core/state/store";
import { BundledFlusterTheme, darkSyntaxThemes, lightSyntaxThemes } from "src/desktop/features/code/data/bundled_themes";
import { setCodeTheme } from "src/desktop/features/code/state/code_state_slice";

class CommandPaletteDarkTheme extends CommandPaletteCategory {
    constructor() {
        super("Dark Mode", "theme-dark-mode");
    }
    filterByLocation(): boolean {
        return true;
    }

    bottomBar(): ReactNode {
        return null;
    }
    async getItems(): Promise<CommandPaletteAnyEntry[]> {
        return darkSyntaxThemes.map((x: BundledFlusterTheme) => {
            return new GeneralCommandPaletteItem(x, `dark_${x}`, async () => {
                store.dispatch(
                    setCodeTheme({
                        themeMode: "dark",
                        value: x,
                    })
                );
            });
        });
    }
}

class CommandPaletteLightTheme extends CommandPaletteCategory {
    constructor() {
        super("Light Mode", "theme-light-mode");
    }

    bottomBar(): ReactNode {
        return null;
    }
    filterByLocation(): boolean {
        return true;
    }
    async getItems(): Promise<CommandPaletteAnyEntry[]> {
        return lightSyntaxThemes.map((x: BundledFlusterTheme) => {
            return new GeneralCommandPaletteItem(x, `light-${x}`, async () => {
                store.dispatch(
                    setCodeTheme({
                        themeMode: "light",
                        value: x,
                    })
                );
            });
        });
    }
}

export class CodeThemeCommandPaletteRoot extends CommandPaletteCategory {
    constructor() {
        super("Syntax Theme", "syntax-theme");
    }

    bottomBar(): ReactNode {
        return null;
    }
    filterByLocation(): boolean {
        return true;
    }
    async getItems(): Promise<CommandPaletteAnyEntry[]> {
        return [new CommandPaletteDarkTheme(), new CommandPaletteLightTheme()];
    }
}
