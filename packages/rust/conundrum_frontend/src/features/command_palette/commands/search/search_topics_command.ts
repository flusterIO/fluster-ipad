import { CommandGroupId } from "../command_group_id";
import { CommandPaletteCommand } from "../command_palette_command";

export class SearchTopicsCommand extends CommandPaletteCommand {
    constructor() {
        super({
            label: "Topics",
            key: "search-topics",
            hasChildren: true,
            groupId: CommandGroupId.Search,
        });
    }
    async children(): Promise<CommandPaletteCommand[]> {
        return [];
    }
}
