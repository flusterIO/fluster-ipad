import { commands } from "@fluster/desktop_bindings";
import { AppRoutes } from "src/desktop/features/navigation/data/app_routes";
import { showToast } from "src/desktop/features/notifications/toasts/actions/show_toast";
import { CommandPaletteAnyEntry } from "../models/command_palette_any_entry";
import { CommandPaletteCategory } from "../models/command_palette_category";
import { GeneralCommandPaletteItem } from "../models/command_palette_item";
import React, { ReactNode } from "react";
import { getMaxPagination } from "src/desktop/features/search/pagination/data/get_max_pagination";
import { showBibEntryDetailsById } from "src/desktop/features/bibliography/data/show_bib_entry_by_id";

export class CitationsCommandPaletteRoot extends CommandPaletteCategory {
    constructor() {
        super("Citations", "cmd-palette-citations");
    }
    filterByLocation(): boolean {
        return true;
    }
    bottomBar(): ReactNode {
        return (
            <div className="text-sm text-muted-foreground w-full flex flex-row justify-end items-center">
                cmd+Enter to open details panel
            </div>
        );
    }
    async getItems(): Promise<CommandPaletteAnyEntry[]> {
        const items = await commands.getBibEntries(null, getMaxPagination());

        if (items.status === "ok") {
            return items.data.map((d) => {
                return new GeneralCommandPaletteItem(
                    d.html_citation,
                    d.id,
                    async (nav) => {
                        const sp = new URLSearchParams();
                        sp.set("by_bib", d.id);
                        nav(`${AppRoutes.search}?${sp.toString()}`);
                    },
                    async () => {
                        showBibEntryDetailsById(d.id);
                    },
                    true
                );
            });
        } else {
            showToast({
                title: "Uh Oh.",
                body: "Something went wrong while searching by citation. Do you have a .bib file configured in your settings?",
                duration: 5000,
                variant: "Error",
            });
            return [];
        }
    }
}
