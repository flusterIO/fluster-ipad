import { AppState } from "@/state/initial_state";
import { Location } from "react-router";

export interface SidePanelItem<T extends any> {
    id: T;
    label: string;
    /// An optional validator function that will hide the item from the select menu when the validator returns false.
    validator?: (state: AppState, location: Location) => boolean;
}

export enum PanelLeftItemId {
    quickNavigation,
}

export const panelLeftItems: SidePanelItem<PanelLeftItemId>[] = [
    {
        id: PanelLeftItemId.quickNavigation,
        label: "Quick Navigation",
    },
] as const;

export interface PanelLeftState {
    open: boolean;
    /// The portion of the viewport dedicated to the left panel. Between 0 and 1
    width: number;
    selectedPanel: SidePanelItem<PanelLeftItemId>;
}
