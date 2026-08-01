export enum CommandGroupId {
    Sync,
    Clean,
    Backup,
}

export const commandGroupIdToGroupLabel = (id: CommandGroupId): string => {
    switch (id) {
        case CommandGroupId.Sync:
            return "Sync";
        case CommandGroupId.Clean:
            return "Clean";
        case CommandGroupId.Backup:
            return "Backup";
    }
};
