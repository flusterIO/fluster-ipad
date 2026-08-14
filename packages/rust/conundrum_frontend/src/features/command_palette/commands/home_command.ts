import { CommandPaletteCommand } from "./command_palette_command";
import { NavigationCommands } from "./navigation/navigation_commands";
import { SearchSubjectsCommand } from "./search/search_subjects_command";
import { SearchTagsCommand } from "./search/search_tags_command";
import { SearchTopicsCommand } from "./search/search_topics_command";
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
    // eslint-disable-next-line @typescript-eslint/require-await
    async children(): Promise<CommandPaletteCommand[]> {
        return [
            new SyncFileSystemWithoutAICommand(),
            new SyncFileSystemWithAICommand(),
            new SearchTagsCommand(),
            new SearchTopicsCommand(),
            new SearchSubjectsCommand(),
            new NavigationCommands()
        ];
    }
}
