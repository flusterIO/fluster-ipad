import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { initialSearchState } from "./initial_search_state";

const searchSlice = createSlice({
    name: "search",
    initialState: initialSearchState,
    reducers: {
        setGlobalQuery(state, action: PayloadAction<string>) {
            state.globalQuery = action.payload;
        },
    },
});

export const { setGlobalQuery } = searchSlice.actions;

export default searchSlice.reducer;
