import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { databaseInitialState } from "./database_initial_state";
import { type DatabasePanelKey } from "../database_panel_key";

const dbSlice = createSlice({
    name: "database",
    initialState: databaseInitialState,
    reducers: {
        setPanelKey(state, action: PayloadAction<DatabasePanelKey>) {
            state.selected_panel_key = action.payload;
        },
    },
});

export const { setPanelKey } = dbSlice.actions;

export default dbSlice.reducer;
