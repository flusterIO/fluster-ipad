
import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { dashboardTypes } from "./settings_state";
import { initialSettingsState } from "./initial_settings_state";

const slice = createSlice({
    name: "settings",
    initialState: initialSettingsState,
    reducers: {
        setRespectGitIgnore(state, action: PayloadAction<boolean>) {
            return {
                ...state,
                useGitIgnore: action.payload,
            };
        },
        setNotesDirectory(state, action: PayloadAction<string>) {
            return {
                ...state,
                notesDirectory: action.payload,
            };
        },
        setDashboardType(
            state,
            action: PayloadAction<(typeof dashboardTypes)[number]>
        ) {
            return {
                ...state,
                dashboardType: action.payload,
            };
        },
        savedStateApplied(state) {
            return {
                ...state,
                hasLoadedSavedState: true,
            };
        },
    },
});

export const {
    setNotesDirectory,
    savedStateApplied,
    setRespectGitIgnore,
    setDashboardType,
} = slice.actions;

export default slice.reducer;
