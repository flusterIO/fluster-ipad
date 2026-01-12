import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { SidePanelItem } from "../../panel_left/state/panel_left_state";
import { initialPanelRightState } from "./initial_panel_right_state";

const slice = createSlice({
    name: "panelRight",
    initialState: initialPanelRightState,
    reducers: {
        togglePanelRight(state) {
            state.open = !state.open;
        },
        setPanelRightOpen(state, action: PayloadAction<boolean>) {
            state.open = action.payload;
        },
        setPanelRightPanel(state, action: PayloadAction<SidePanelItem>) {
            state.selectedPanel = action.payload;
        },
    },
});

export const { togglePanelRight, setPanelRightOpen, setPanelRightPanel } =
    slice.actions;

export default slice.reducer;
