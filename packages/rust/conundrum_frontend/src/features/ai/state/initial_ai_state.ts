import store from "@/state/store";
import dayjs from "dayjs";
import { type AIState } from "./ai_state";
import { v4 } from "uuid";

export const initialAIState: AIState = {
    chatAgentID: null,
    mostRecentChat: null,
    dailyChatTimeExpires: 60 * 60 * 3 * 1000, // 3am
    dailyChat: null,
};

export const newDailyChat = (
    convo_id: string | null,
): NonNullable<AIState["dailyChat"]> => {
    const now = dayjs();
    const endOfDay = now.endOf("day");
    const timeOffset =
        store.getState().ai?.dailyChatTimeExpires ?? 3 * 60 * 60 * 1000; // Defaults to 3am;
    const expires_at = new Date(endOfDay.valueOf() + timeOffset).toISOString();
    return {
        chat_id: convo_id ?? v4(),
        expires_at,
        was_directed: false,
    };
};
