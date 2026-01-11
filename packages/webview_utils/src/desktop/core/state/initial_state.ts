import { SettingsState } from "src/desktop/features/settings/state/settings_state";
import {
    initialScaffoldState,
    ScaffoldState,
} from "../../features/scaffold/main_scaffold/state/main_scaffold_initial_state";
import { initialSettingsState } from "src/desktop/features/settings/state/initial_settings_state";
import { BibliographyState } from "src/desktop/features/bibliography/state/bib_state";
import { initialBibliographyState } from "src/desktop/features/bibliography/state/initial_bib_state";
import { CodeState } from "src/desktop/features/code/state/code_state";
import { initialCodeState } from "src/desktop/features/code/state/initial_code_state";
import { PanelLeftState } from "src/desktop/features/panel_left/state/initial_panel_left_state";
import { PanelRightState } from "src/desktop/features/panel_right/state/panel_right_state";
import { initialPanelLeftState } from "src/desktop/features/panel_left/state/panel_left_state";
import { initialPanelRightState } from "src/desktop/features/panel_right/state/initial_panel_right_state";

export interface AppState {
    scaffold: ScaffoldState;
    settings: SettingsState;
    bib: BibliographyState;
    code: CodeState;
    panelLeft: PanelLeftState;
    panelRight: PanelRightState
}

export const initialAppState: AppState = {
    scaffold: initialScaffoldState,
    settings: initialSettingsState,
    bib: initialBibliographyState,
    code: initialCodeState,
    panelLeft: initialPanelLeftState,
    panelRight: initialPanelRightState
};
