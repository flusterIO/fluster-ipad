import { CommandGroupId } from "../command_group_id";
import { CommandPaletteCommand } from "../command_palette_command";

export class SearchSubjectsCommand extends CommandPaletteCommand {
    constructor() {
        super({
            label: "Subjects",
            key: "search-subjects",
            hasChildren: true,
            groupId: CommandGroupId.Search,
        });
    }
    async children(): Promise<CommandPaletteCommand[]> {
        return [];
    }
}
