import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { initialScaffoldState } from "./initial_scaffold_state";
import { AppLoadingState } from "./scaffold_state";

const slice = createSlice({
    name: "scaffold",
    initialState: initialScaffoldState,
    reducers: {
        appendLoadingState(state, action: PayloadAction<AppLoadingState>) {
            state.loading = [...state.loading, action.payload];
        },
        removeLoadingState(state, action: PayloadAction<AppLoadingState>) {
            state.loading = state.loading.filter((l) => l != action.payload);
        },
    },
});

export const { appendLoadingState, removeLoadingState } = slice.actions;

export default slice.reducer;
