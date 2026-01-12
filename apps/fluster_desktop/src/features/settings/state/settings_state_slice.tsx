import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { initialSettingsState } from "./initial_settings_state";

const slice = createSlice({
    name: "settings",
    initialState: initialSettingsState,
    reducers: {
        /// Call when app has loaded to avoid saving initial state to DB.
        savedStateApplied(state) {
            state.hasLoadedSavedState = true;
        },
    },
});

export const { savedStateApplied } = slice.actions;

export default slice.reducer;
