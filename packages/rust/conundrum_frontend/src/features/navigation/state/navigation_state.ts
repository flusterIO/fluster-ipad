import { type SecondaryPanelKey } from "../secondary_panel/secondary_panel_key";

interface SidePanelState {
    active_panel: SecondaryPanelKey;
}

export interface NavigationState {
    loading: boolean;
    side_panel: SidePanelState;
}
