import { CommandPaletteCommand } from "./command_palette_command";
import {
    SyncFileSystemWithAICommand,
    SyncFileSystemWithoutAICommand,
} from "./sync/sync_filesystem";

export class HomeCOmmandPaletteCommand extends CommandPaletteCommand {
    constructor() {
        super({
            label: "Home",
            key: "home-command",
            hasChildren: true,
        });
    }
    async children(): Promise<CommandPaletteCommand[]> {
        return [
            new SyncFileSystemWithoutAICommand(),
            new SyncFileSystemWithAICommand(),
        ];
    }
}
