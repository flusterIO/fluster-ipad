import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { initialPanelLeftState } from "./initial_panel_left_state";
import { SidePanelItem } from "./panel_left_state";

const slice = createSlice({
    name: "panelLeft",
    initialState: initialPanelLeftState,
    reducers: {
        togglePanelLeft(state) {
            state.open = !state.open;
        },
        setPanelLeftOpen(state, action: PayloadAction<boolean>) {
            state.open = action.payload;
        },
        setPanelLeftPanel(state, action: PayloadAction<SidePanelItem>) {
            state.selectedPanel = action.payload;
        },
    },
});

export const { togglePanelLeft, setPanelLeftOpen, setPanelLeftPanel } =
    slice.actions;

export default slice.reducer;
