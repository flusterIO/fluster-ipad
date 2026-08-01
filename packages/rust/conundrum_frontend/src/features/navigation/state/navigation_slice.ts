import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { initialNavigationState } from "./initial_navigation_state";
import { HomeCOmmandPaletteCommand } from "#/command_palette/commands/home_command";

const navigationSlice = createSlice({
    name: "navigation",
    initialState: initialNavigationState,
    reducers: {
        setLoading(state, action: PayloadAction<boolean>) {
            state.loading = action.payload;
        },
        setCommandPaletteOpen(state, action: PayloadAction<boolean | "toggle">) {
            const currentlyOpen = Boolean(state.commandPalette.length);
            if (!currentlyOpen && (action.payload === "toggle" || action.payload)) {
                state.commandPalette = [new HomeCOmmandPaletteCommand()];
            } else {
                return {
                    ...state,
                };
            }
        },
    },
});

export const { setLoading, setCommandPaletteOpen } = navigationSlice.actions;

export default navigationSlice.reducer;
