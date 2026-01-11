import { CommandPaletteAnyEntry } from "../models/command_palette_any_entry";
import { CommandPaletteCategory } from "../models/command_palette_category";
import { ReactNode } from "react";
import dayjs from "dayjs";
import { GeneralCommandPaletteItem } from "../models/command_palette_item";
import { MdxNoteGroup, commands } from "@fluster/desktop_bindings";
import { AppRoutes } from "src/desktop/features/navigation/data/app_routes";

export const getRecentlyAccessedNotes = async (): Promise<MdxNoteGroup[]> => {
    const res = await commands.getRecentlyAccessedNotes();
    if (res.status === "ok") {
        return res.data
            .filter(
                (a) =>
                    dayjs(a.mdx.last_read, {
                        utc: true,
                    }).valueOf() > 0
            )
            .sort((a: MdxNoteGroup, b: MdxNoteGroup) => {
                return (
                    dayjs(b.mdx.last_read, {
                        utc: true,
                    }).valueOf() -
                    dayjs(a.mdx.last_read, {
                        utc: true,
                    }).valueOf()
                );
            });
    } else {
        return [];
    }
};

export class RecentlyAccessedCommandPaletteRoot extends CommandPaletteCategory {
    constructor() {
        super("Recently Accessed Notes", "cmd-palette-recently-accessed-notes");
    }
    filterByLocation(): boolean {
        return true;
    }
    bottomBar(): ReactNode {
        return null;
    }
    async getItems(): Promise<CommandPaletteAnyEntry[]> {
        const notes = await getRecentlyAccessedNotes();
        if (notes) {
            return notes?.map((item: MdxNoteGroup) => {
                return new GeneralCommandPaletteItem(
                    item.front_matter.title,
                    `recent-visited-${item.mdx.file_path}`,
                    async (nav) => {
                        const sp = new URLSearchParams();
                        sp.set("fsPath", item.mdx.file_path);
                        nav(`${AppRoutes.viewMdxNote}?${sp.toString()}`);
                    },
                    undefined,
                    undefined
                );
            });
        }
        return [];
    }
}
