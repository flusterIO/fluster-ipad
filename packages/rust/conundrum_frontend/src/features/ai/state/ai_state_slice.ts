import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { initialAIState } from "./initial_ai_state";

const aiSlice = createSlice({
    name: "ai",
    initialState: initialAIState,
    reducers: {
        setChatAgentID(state, action: PayloadAction<string | null>) {
            state.chatAgentID = action.payload;
        },
    },
});

export const { setChatAgentID } = aiSlice.actions;

export default aiSlice.reducer;
