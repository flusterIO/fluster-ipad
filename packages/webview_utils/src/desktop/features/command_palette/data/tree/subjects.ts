import { commands } from "@fluster/desktop_bindings";
import { getSubjectUrl } from "src/desktop/core/path_utils";
import { CommandPaletteAnyEntry } from "../models/command_palette_any_entry";
import { CommandPaletteCategory } from "../models/command_palette_category";
import { GeneralCommandPaletteItem } from "../models/command_palette_item";
import { ReactNode } from "react";

export class SubjectsCommandPaletteRoot extends CommandPaletteCategory {
    constructor() {
        super("Subject", "search-by-subject");
    }
    filterByLocation(): boolean {
        return true;
    }
    bottomBar(): ReactNode {
        return null;
    }
    async getItems(): Promise<CommandPaletteAnyEntry[]> {
        const res = await commands.getAllSubjects();
        if (res.status === "ok") {
            return res.data.map((x) => {
                return new GeneralCommandPaletteItem(x.value, x.value, async (nav) => {
                    nav(getSubjectUrl(x.value));
                });
            });
        } else {
            console.error("An error occurred while gathering tags: ", res.error);
            return [];
        }
    }
}
