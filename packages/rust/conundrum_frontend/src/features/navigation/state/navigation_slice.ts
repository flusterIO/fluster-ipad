import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { initialNavigationState } from "./initial_navigation_state";
import { type SecondaryPanelKey } from "../secondary_panel/secondary_panel_key";

const navigationSlice = createSlice({
    name: "navigation",
    initialState: initialNavigationState,
    reducers: {
        setLoading(state, action: PayloadAction<boolean>) {
            state.loading = action.payload;
        },
        setSidePanelOpen(state, action: PayloadAction<boolean | "toggle">) {
            state.side_panel.open =
                typeof action.payload === "boolean"
                    ? action.payload
                    : !state.side_panel.open;
        },
        setSecondaryActivePanel(state, action: PayloadAction<SecondaryPanelKey>) {
            state.side_panel.active_panel = action.payload;
        },
    },
});

export const { setLoading, setSidePanelOpen, setSecondaryActivePanel } =
    navigationSlice.actions;

export default navigationSlice.reducer;
