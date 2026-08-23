import { useEffect } from "react";
import { useSearchParams } from "react-router";
import { useDispatch } from "react-redux";
import {
    setDailyChatViewed,
    setMostRecentChat,
} from "#/ai/state/ai_state_slice";
import { useSelector } from "react-redux";
import { type AppState } from "@/state/initial_state";

/**
 * A required observer on the chat page that set's some timers and things based on the chat session id.
 */
export const useObserveChatLocationState = () => {
    const [sp] = useSearchParams();
    const convo = sp.get("convo");
    const dispatch = useDispatch();
    const dailyChat = useSelector((state: AppState) => {
        return state.ai.dailyChat;
    });
    useEffect(() => {
        if (convo) {
            dispatch(setMostRecentChat(convo));
            if (convo === dailyChat?.chat_id) {
                dispatch(setDailyChatViewed(true));
            }
        }
    }, [convo]);
    return null;
};

useObserveChatLocationState.displayName = "useObserveChatLocationState";
