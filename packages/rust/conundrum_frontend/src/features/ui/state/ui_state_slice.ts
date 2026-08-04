import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import {initialUIState} from "./initial_ui_state"

const uiSlice = createSlice({
    name: "ui",
    initialState: initialUIState,
    reducers: {
        setSyntaxTheme(state, action: PayloadAction<typeof initialUIState["syntaxTheme"]>) {
            state.syntaxTheme = action.payload;
        },
    },
});

export const { setSyntaxTheme } = uiSlice.actions;

export default uiSlice.reducer;
