import { client } from "@/app/rspc_client";
import { type UserWorkspace } from "@/codegen/bindings";
import { type NavigateFunction } from "react-router";
import { CommandGroupId } from "../command_group_id";
import { CommandPaletteCommand } from "../command_palette_command";

const WORKSPACE_PAGE_SIZE = 100;

class SyncWorkspaceCommand extends CommandPaletteCommand {
    private readonly root: string;

    constructor(root: string, label: string) {
        super({
            hasChildren: false,
            label,
            key: `sync-workspace-${root}`,
            groupId: CommandGroupId.Sync,
        });
        this.root = root;
    }

    async act(_nav: NavigateFunction): Promise<void> {
        await client.mutation(["sync.workspace", this.root]);
    }
}

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
            hasChildren: true,
            label: "Sync",
            key: "sync-with-ai",
            groupId: CommandGroupId.Sync,
        });
    }

    async children(): Promise<CommandPaletteCommand[]> {
        const workspaces: UserWorkspace[] = [];
        for (let page = 1; ; page += 1) {
            const pageItems = await client.query([
                "crud.user_workspace.get_by_predicate",
                {
                    predicate: null,
                    pagination: {
                        page,
                        per_page: WORKSPACE_PAGE_SIZE,
                    },
                },
            ]);
            workspaces.push(...pageItems);
            if (pageItems.length < WORKSPACE_PAGE_SIZE) {
                break;
            }
        }

        return workspaces.map(
            (workspace) =>
                new SyncWorkspaceCommand(
                    workspace.root,
                    workspace.label?.trim() || workspace.root,
                ),
        );
    }
}
