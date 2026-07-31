import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { initialNavigationState } from "./initial_navigation_state";

const navigationSlice = createSlice({
    name: "navigation",
    initialState: initialNavigationState,
    reducers: {
        setLoading(state, action: PayloadAction<boolean>) {
            state.loading = action.payload;
        },
    },
});

export const { setLoading } = navigationSlice.actions;

export default navigationSlice.reducer;
