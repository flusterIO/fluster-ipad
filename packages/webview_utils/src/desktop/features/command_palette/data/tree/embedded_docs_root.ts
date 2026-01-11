import { ReactNode } from "react";
import { CommandPaletteAnyEntry } from "../models/command_palette_any_entry";
import { CommandPaletteCategory } from "../models/command_palette_category";
import { GeneralCommandPaletteItem } from "../models/command_palette_item";
import { ComponentDocsCommandPaletteRoot } from "./component_docs";
import { CommandPaletteEntryWithDocId } from "../models/command_palette_entry_with_doc_id";
import { InternalEmbeddedDocsId } from "@fluster/desktop_bindings";
import { DocsByIdPreview } from "../../presentation/previews/doc_by_id_preview";
import { AppRoutes } from "src/desktop/features/navigation/data/app_routes";
import { embeddedDocLabels } from "src/desktop/features/docs/data/embedded_docs_labels";

export class EmbeddedDocsCommandPaletteRoot extends CommandPaletteCategory {
    constructor() {
        super("Documentation", "cmd-palette-docs", DocsByIdPreview);
    }
    filterByLocation(): boolean {
        return true;
    }
    bottomBar(): ReactNode {
        return null;
    }
    async getItems(): Promise<CommandPaletteAnyEntry[]> {
        const items: CommandPaletteAnyEntry[] = Object.entries(
            embeddedDocLabels
        ).map((k) => {
            return new CommandPaletteEntryWithDocId(
                k[1],
                k[1],
                async (nav) => {
                    nav(`${AppRoutes.embeddedDocs.toString()}/${encodeURI(k[0])}`);
                },
                null,
                k[0] as InternalEmbeddedDocsId
            );
        });

        items.push(new ComponentDocsCommandPaletteRoot());
        // items.push(
        //     new GeneralCommandPaletteItem(
        //         "Constants",
        //         "constants-docs",
        //         async (nav) => nav(AppRoutes.constantsTable)
        //     )
        // );
        items.push(
            new GeneralCommandPaletteItem(
                "View all component docs",
                "all-component-docs",
                async (nav) => nav(AppRoutes.embeddedDocs)
            )
        );
        return items;
    }
}
