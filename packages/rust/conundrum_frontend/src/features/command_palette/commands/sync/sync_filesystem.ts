import { CommandGroupId } from "../command_group_id";
import { CommandPaletteCommand } from "../command_palette_command";

export class SyncFileSystemWithoutAICommand extends CommandPaletteCommand {
    constructor() {
        super({
            hasChildren: false,
            label: "Sync (without AI)",
            key: "sync-without-ai",
            groupId: CommandGroupId.Sync,
        });
    }
}

export class SyncFileSystemWithAICommand extends CommandPaletteCommand {
    constructor() {
        super({
            hasChildren: false,
            label: "Sync",
            key: "sync-with-ai",
            groupId: CommandGroupId.Sync,
        });
    }
}
