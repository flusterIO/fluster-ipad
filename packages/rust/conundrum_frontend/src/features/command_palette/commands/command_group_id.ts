export enum CommandGroupId {
    Sync = "sync",
    Clean = "clean",
    Backup = "backup",
    Search = "search",
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
    }
};
