import { type NavigateFunction } from "react-router";
import { CommandGroupId } from "../command_group_id";
import { CommandPaletteCommand } from "../command_palette_command";
import { AppPaths } from "#/navigation/app_paths";

interface NavigationItem {
    label: string
    href: string
}

class NavigationCommand extends CommandPaletteCommand {
    href: string
    constructor({ label, href }: { label: string, href: string }) {
        super({
            label,
            hasChildren: false,
            key: label
        })
        this.href = href
    }
    async act(nav: NavigateFunction): Promise<void> {
        await nav(this.href);
    }
}

export class NavigationCommands extends CommandPaletteCommand {
    constructor() {
        super({
            label: "Navigation",
            key: "navigation-commands",
            hasChildren: true,
            groupId: CommandGroupId.Navigation,
        });
    }

    getNavigationItems(): NavigationItem[] {
        return [
            {
                href: AppPaths.health,
                label: "Go to Health page"
            },
            {
                href: AppPaths.search,
                label: "Go to Search page"
            },

            {
                href: AppPaths.logs,
                label: "Logs"
            },

        ]
    }
    async children(): Promise<CommandPaletteCommand[]> {
        return this.getNavigationItems().map((item) => {
            return new NavigationCommand(item)
        })
    }
}
