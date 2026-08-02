import { CommandGroupId } from "../command_group_id";
import { CommandPaletteCommand } from "../command_palette_command";

export class SearchTagsCommand extends CommandPaletteCommand {
    constructor() {
        super({
            label: "Tags",
            key: "search-tags",
            hasChildren: true,
            groupId: CommandGroupId.Search,
        });
    }
    async children(): Promise<CommandPaletteCommand[]> {
        return [];
    }
}
