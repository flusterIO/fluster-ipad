import { NavigateFunction } from "react-router";
import { CommandPaletteAnyEntry } from "../models/command_palette_any_entry";
import { CommandPaletteCategory } from "../models/command_palette_category";
import { ReactNode } from "react";
import { CommandPaletteEntryWithPreview } from "../models/command_palette_entry_with_preview";
import { AppRoutes } from "src/desktop/features/navigation/data/app_routes";
import { ComponentDocsPreview } from "../../presentation/previews/component_docs_preview";
import { componentDocItems } from "../../../docs/data/embedded_doc_items"

export class ComponentDocsCommandPaletteRoot extends CommandPaletteCategory {
    constructor() {
        super("Components", "cmd-palette-component-docs", ComponentDocsPreview);
    }
    filterByLocation(): boolean {
        return true;
    }
    bottomBar(): ReactNode {
        return null;
    }
    async getItems(): Promise<CommandPaletteAnyEntry[]> {
        return componentDocItems.map((c) => {
            return new CommandPaletteEntryWithPreview(
                c.label,
                `component-${c.fp}`,
                async (nav: NavigateFunction) => {
                    const sp = new URLSearchParams();
                    sp.set("fsPath", c.fp);
                    nav(`${AppRoutes.embeddedDocs}?${sp.toString()}`);
                },
                null,
                c.fp
            );
        });
    }
}
