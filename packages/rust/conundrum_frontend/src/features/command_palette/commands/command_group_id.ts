export enum CommandGroupId {
    Sync = "sync",
    Clean = "clean",
    Backup = "backup",
    Search = "search",
    Navigation = "navigation"
}

export const commandGroupIdToGroupLabel = (id: CommandGroupId): string => {
    switch (id) {
        case CommandGroupId.Sync:
            return "Sync";
        case CommandGroupId.Clean:
            return "Clean";
        case CommandGroupId.Backup:
            return "Backup";
        case CommandGroupId.Search:
            return "Search";
        case CommandGroupId.Navigation:
            return "Navigation";
    }
};

export const commandPaletteGroupIdToKeywords = (
    id?: CommandGroupId,
    additionalKeywords: string[] = [],
) => {
    if (!id) {
        return additionalKeywords;
    }
    return [
        ...{
            [CommandGroupId.Search]: ["search"],
            [CommandGroupId.Clean]: ["clean"],
            [CommandGroupId.Backup]: ["backup", "restore"],
            [CommandGroupId.Sync]: ["sync"],
            [CommandGroupId.Navigation]: ["navigation", "jump", "to"],
        }[id],
        ...additionalKeywords,
    ];
};
