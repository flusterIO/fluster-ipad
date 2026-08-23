import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { initialAIState, newDailyChat } from "./initial_ai_state";
import { type useDispatch } from "react-redux";

const aiSlice = createSlice({
    name: "ai",
    initialState: initialAIState,
    reducers: {
        setChatAgentID(state, action: PayloadAction<string | null>) {
            state.chatAgentID = action.payload;
        },
        setDailyChat(
            state,
            action: PayloadAction<typeof initialAIState.dailyChat>,
        ) {
            state.dailyChat = action.payload;
        },
        setDailyChatViewed(state, action: PayloadAction<boolean>) {
            state.dailyChat ??= newDailyChat(null);
            state.dailyChat.was_directed = action.payload;
        },
        setMostRecentChat(state, action: PayloadAction<string>) {
            state.mostRecentChat = action.payload;
        },
    },
});

const { setDailyChat, ...props } = aiSlice.actions;

export const { setChatAgentID, setMostRecentChat, setDailyChatViewed } = props;

export const resetDailyChat = (dispatch: ReturnType<typeof useDispatch>) => {
    dispatch(setDailyChat(newDailyChat(null)));
};

export default aiSlice.reducer;
