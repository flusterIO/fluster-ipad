import { supportedTheme } from "src/desktop/core/core_types/supported_themes";
import { CommandPaletteAnyEntry } from "../models/command_palette_any_entry";
import { CommandPaletteCategory } from "../models/command_palette_category";
import { GeneralCommandPaletteItem } from "../models/command_palette_item";
import { ReactNode } from "react";
import { setTheme } from "src/desktop/features/scaffold/main_scaffold/main_scaffold_actions";

export class ThemeCommandPaletteRoot extends CommandPaletteCategory {
    constructor() {
        super("Themes", "cmd-palette-themes");
    }
    filterByLocation(): boolean {
        return true;
    }
    bottomBar(): ReactNode {
        return null;
    }
    async getItems(): Promise<CommandPaletteAnyEntry[]> {
        return supportedTheme.map(
            (x) =>
                new GeneralCommandPaletteItem(x, `theme-${x}`, async () => {
                    setTheme(x);
                })
        );
    }
}
