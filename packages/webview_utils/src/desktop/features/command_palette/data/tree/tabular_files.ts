import { CommandPaletteAnyEntry } from "../models/command_palette_any_entry";
import { CommandPaletteCategory } from "../models/command_palette_category";
import { GeneralCommandPaletteItem } from "../models/command_palette_item";
import { NavigateFunction } from "react-router";
import { ReactNode } from "react";
import { commands } from "@fluster/desktop_bindings";
import { AppState } from "src/desktop/core/state/initial_state";
import store from "src/desktop/core/state/store";
import { showToast } from "src/desktop/features/notifications/toasts/actions/show_toast";
import { getTabularDataTableUrl } from "src/desktop/core/path_utils";

export class TabularFilesCommandPaletteRoot extends CommandPaletteCategory {
    constructor() {
        super("Tabular Files", "tabular");
    }
    filterByLocation(): boolean {
        return true;
    }
    bottomBar(): ReactNode {
        return null;
    }
    async getItems(): Promise<CommandPaletteAnyEntry[]> {
        const coreState: AppState["settings"] = (store.getState() as AppState).settings;
        const res = await commands.getFilesByFileExtensions(
            ["csv"],
            coreState.notesDirectory,
            coreState.nThreads.toString()
        );
        if (res.status === "error") {
            showToast({
                title: "An error occurred",
                body: "An error occurred while attempting to gather your tabular files. If this continues, please file an issue on Github.",
                variant: "Error",
                duration: 5000,
            });
            return [];
        }
        return res.data.map((s) => {
            const item = new GeneralCommandPaletteItem(
                coreState.notesDirectory.length &&
                    s.startsWith(coreState.notesDirectory)
                    ? s.replace(coreState.notesDirectory, "")
                    : s,
                `tabular-${s}`,
                async (nav: NavigateFunction) => {
                    nav(getTabularDataTableUrl(s));
                }
            );
            item.itemClasses = "text-sm [&_p]:text-sm";
            return item;
        });
    }
}
