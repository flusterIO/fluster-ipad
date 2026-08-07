import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { databaseInitialState } from "./database_initial_state";
import { type DBAuth } from "./database_state";

const dbSlice = createSlice({
    name: "database",
    initialState: databaseInitialState,
    reducers: {
        setDBAuth(state, action: PayloadAction<DBAuth>) {
            state.auth = action.payload;
        },
    },
});

export const { setDBAuth } = dbSlice.actions;

export default dbSlice.reducer;
