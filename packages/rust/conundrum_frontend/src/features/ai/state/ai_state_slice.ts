import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { initialAIState } from "./initial_ai_state";
import { type useDispatch } from "react-redux";
import { v4 } from "uuid";
import { Dayjs } from "dayjs";
import store from "@/state/store";

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
    },
});

const { setDailyChat, ...props } = aiSlice.actions;

export const { setChatAgentID } = props;

export const resetDailyChat = (dispatch: ReturnType<typeof useDispatch>) => {
    const now = new Dayjs();
    const endOfDay = now.endOf("day");
    const timeOffset =
        store.getState().ai?.dailyChatTimeExpires ?? 3 * 60 * 60 * 1000; // Defaults to 3am;
    const expires_at = new Date(endOfDay.valueOf() + timeOffset).toISOString();
    dispatch(
        setDailyChat({
            chat_id: v4(),
            expires_at,
        }),
    );
};

export default aiSlice.reducer;
