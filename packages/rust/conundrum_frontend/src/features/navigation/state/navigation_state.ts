import { type CommandPaletteCommand } from "#/command_palette/commands/command_palette_command";

export interface NavigationState {
    loading: boolean;
    commandPalette: CommandPaletteCommand[];
}
