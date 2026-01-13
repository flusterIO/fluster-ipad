import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { initialPanelLeftState } from "./initial_panel_left_state";
import { PanelLeftItemId, SidePanelItem } from "./panel_left_state";

const slice = createSlice({
    name: "panelLeft",
    initialState: initialPanelLeftState,
    reducers: {
        setPanelLeftPortion(state, action: PayloadAction<number>) {
            state.width = action.payload;
        },
        setPanelLeftOpen(state, action: PayloadAction<boolean | "toggle">) {
            state.open = action.payload === "toggle" ? !state.open : action.payload;
        },
        setPanelLeftPanel(
            state,
            action: PayloadAction<SidePanelItem<PanelLeftItemId>>,
        ) {
            state.selectedPanel = action.payload;
        },
    },
});

export const { setPanelLeftPortion, setPanelLeftOpen, setPanelLeftPanel } =
    slice.actions;

export default slice.reducer;
