import { panelLeftItems, PanelLeftState } from "./panel_left_state";

export const initialPanelLeftState: PanelLeftState = {
    open: false,
    width: 0.2,
    selectedPanel: panelLeftItems[0],
};
