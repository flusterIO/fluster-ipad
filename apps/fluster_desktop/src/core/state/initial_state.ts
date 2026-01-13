import { CommandPaletteState } from "#/command_palette/state/command_palette_state";
import { initialCommandPaletteState } from "#/command_palette/state/initial_command_palette_state";
import { initialKeymapState } from "#/keymap/state/initial_keymap_state";
import { KeymapState } from "#/keymap/state/keymap_state";
import { initialScaffoldState } from "#/scaffold/state/initial_scaffold_state";
import { ScaffoldState } from "#/scaffold/state/scaffold_state";
import { initialPanelLeftState } from "../../features/panel_left/state/initial_panel_left_state";
import { PanelLeftState } from "../../features/panel_left/state/panel_left_state";
import { initialPanelRightState } from "../../features/panel_right/state/initial_panel_right_state";
import { PanelRightState } from "../../features/panel_right/state/panel_right_state";
import { initialSettingsState } from "../../features/settings/state/initial_settings_state";
import { SettingsState } from "../../features/settings/state/settings_state";

export interface AppState {
    settings: SettingsState;
    panelLeft: PanelLeftState;
    panelRight: PanelRightState;
    scaffold: ScaffoldState;
    commandPalette: CommandPaletteState;
    keymap: KeymapState;
}

export const initialAppState: AppState = {
    settings: initialSettingsState,
    panelLeft: initialPanelLeftState,
    panelRight: initialPanelRightState,
    scaffold: initialScaffoldState,
    commandPalette: initialCommandPaletteState,
    keymap: initialKeymapState,
};
