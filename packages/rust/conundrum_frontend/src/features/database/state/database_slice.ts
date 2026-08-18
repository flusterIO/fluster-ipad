import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { databaseInitialState } from "./database_initial_state";
import { type BackendStatus } from "../db_utility_types/health";

const dbSlice = createSlice({
    name: "database",
    initialState: databaseInitialState,
    reducers: {
        setBackendStatus(state, action: PayloadAction<BackendStatus>) {
            state.backend_status = action.payload;
        },
    },
});

export const { setBackendStatus } = dbSlice.actions;

export default dbSlice.reducer;
