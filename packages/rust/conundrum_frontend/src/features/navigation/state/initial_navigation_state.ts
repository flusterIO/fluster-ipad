import { SecondaryPanelKey } from "../secondary_panel/secondary_panel_key";
import { type NavigationState } from "./navigation_state";

export const initialNavigationState: NavigationState = {
    loading: false,
    side_panel: {
        open: false,
        active_panel: SecondaryPanelKey.ChatSelect,
    },
};
