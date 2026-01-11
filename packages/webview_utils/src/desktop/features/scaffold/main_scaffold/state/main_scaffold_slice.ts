import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { initialScaffoldState, ThemeMode } from "./main_scaffold_initial_state";
import { supportedTheme } from "src/desktop/core/core_types/supported_themes";

const scaffoldSlice = createSlice({
    name: "scaffold",
    initialState: initialScaffoldState,
    reducers: {
        setThemeMode(state, action: PayloadAction<ThemeMode>) {
            state.themeMode = action.payload;
        },
        setTheme(state, action: PayloadAction<(typeof supportedTheme)[number]>) {
            state.theme = action.payload;
        },
    },
});

export const { setThemeMode: setThemeModeAction, setTheme: setThemeAction } =
    scaffoldSlice.actions;

export default scaffoldSlice.reducer;
