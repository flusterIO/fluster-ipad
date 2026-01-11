import { CommandPaletteAnyEntry } from "../models/command_palette_any_entry";
import { CommandPaletteCategory } from "../models/command_palette_category";
import { GeneralCommandPaletteItem } from "../models/command_palette_item";
import { NavigateFunction } from "react-router";
import { ReactNode } from "react";
import { AppState } from "src/desktop/core/state/initial_state";
import store from "src/desktop/core/state/store";
import { fsFileExtensionGlob } from "src/desktop/features/search/data/methods/file_extension_glob";
import { getHtmlFileURl } from "src/desktop/core/path_utils";

export class HtmlFilesCommandPaletteRoot extends CommandPaletteCategory {
    constructor() {
        super("Html files", "html_files");
    }
    filterByLocation(): boolean {
        return true;
    }
    bottomBar(): ReactNode {
        return null;
    }
    async getItems(): Promise<CommandPaletteAnyEntry[]> {
        const res = await fsFileExtensionGlob("html");
        const notesDir = (store.getState() as AppState).settings.notesDirectory ?? "";
        return res.map((s) => {
            const item = new GeneralCommandPaletteItem(
                notesDir.length && s.startsWith(notesDir) ? s.replace(notesDir, "") : s,
                `pdf-${s}`,
                async (nav: NavigateFunction) => {
                    nav(getHtmlFileURl(s));
                }
            );
            item.itemClasses = "text-sm [&_p]:text-sm";
            return item;
        });
    }
}
