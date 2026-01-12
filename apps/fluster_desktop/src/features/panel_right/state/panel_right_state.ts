import { SidePanelItem } from "../../panel_left/state/panel_left_state";

export enum PanelRightItemId {
    quickSettings,
}

export const panelRightItems: SidePanelItem<PanelRightItemId>[] = [
    {
        id: PanelRightItemId.quickSettings,
        label: "Test Panel",
    },
] as const;

export interface PanelRightState {
    open: boolean;
    /// The portion of the viewport dedicated to the left panel. Between 0 and 1
    width: number;
    selectedPanel: SidePanelItem<PanelRightItemId>;
}
