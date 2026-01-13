import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { initialCommandPaletteState } from "./initial_command_palette_state";

const slice = createSlice({
    name: "panelLeft",
    initialState: initialCommandPaletteState,
    reducers: {
        setCommandPaleteOpen(state, action: PayloadAction<boolean | "toggle">) {
            state.open = action.payload === "toggle" ? !state.open : action.payload;
        },
    },
});

export const { setCommandPaleteOpen } = slice.actions;

export default slice.reducer;
