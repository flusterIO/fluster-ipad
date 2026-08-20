import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { initialAIState } from "./initial_ai_state";
import { v4 } from "uuid";

const aiSlice = createSlice({
    name: "ai",
    initialState: initialAIState,
    reducers: {
        setChatAgentID(state, action: PayloadAction<string | null>) {
            state.chatAgentID = action.payload;
        },
        resetDailyChat(state, action: PayloadAction<null>) {
            state.dailyChat = {
                chat_id: v4(),
                expires_at: new Date(
                    new Date().valueOf() + 86_400 * 1000,
                ).toUTCString(),
            };
        },
    },
});

export const { setChatAgentID, resetDailyChat } = aiSlice.actions;

export default aiSlice.reducer;
