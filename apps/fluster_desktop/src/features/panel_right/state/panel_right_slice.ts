import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import {
    PanelLeftItemId,
    SidePanelItem,
} from "../../panel_left/state/panel_left_state";
import { initialPanelRightState } from "./initial_panel_right_state";
import { PanelRightItemId } from "./panel_right_state";

const slice = createSlice({
    name: "panelRight",
    initialState: initialPanelRightState,
    reducers: {
        setPanelRightPortion(state, action: PayloadAction<number>) {
            state.width = action.payload;
        },
        setPanelRightOpen(state, action: PayloadAction<boolean | "toggle">) {
            state.open = action.payload === "toggle" ? !state.open : action.payload;
        },
        setPanelRightPanel(
            state,
            action: PayloadAction<SidePanelItem<PanelRightItemId>>,
        ) {
            state.selectedPanel = action.payload;
        },
    },
});

export const { setPanelRightPortion, setPanelRightOpen, setPanelRightPanel } =
    slice.actions;

export default slice.reducer;
