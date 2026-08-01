import { type CommandGroupId } from "./command_group_id";

/* eslint-disable @typescript-eslint/require-await */
interface CommandPaletteCommandProps {
    label: string;
    /// Some unique key
    key: string;
    hasChildren: boolean;
    groupId?: CommandGroupId;
}

export abstract class CommandPaletteCommand implements CommandPaletteCommandProps {
    label: string;
    key: string;
    hasChildren: boolean;
    groupId?: CommandGroupId;
    constructor(props: CommandPaletteCommandProps) {
        this.label = props.label;
        this.key = props.key;
        this.hasChildren = props.hasChildren;
        this.groupId = props.groupId;
    }

    async act(): Promise<void> {
        throw new Error("Method not yet implemented");
    }

    async children(): Promise<CommandPaletteCommand[]> {
        return [];
    }

    emptyText(): string {
        return "No results to show";
    }
}
