import { resetDailyChat } from "#/ai/state/ai_state_slice";
import { type AppState } from "@/state/initial_state";
import { useEffect, useRef, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

export const useTimer = (exec_at: Date | null, cb: () => void) => {
    const timer = useRef<NodeJS.Timeout | null>(null);

    useEffect(() => {
        if (!exec_at) {
            return;
        }
        const target = exec_at.valueOf();
        const now = new Date().valueOf();
        const diff = target - now;
        if (diff <= 0) {
            return;
        }
        timer.current = setTimeout(cb, diff);
        return () => {
            if (timer.current) {
                clearTimeout(timer.current);
            }
        };
    }, [exec_at, cb]);
};

export const useGlobalTimers = () => {
    const aiDailyChat = useSelector((state: AppState) => {
        return state.ai.dailyChat?.expires_at
            ? new Date(state.ai.dailyChat.expires_at)
            : null;
    });

    const dispatch = useDispatch();
    useTimer(aiDailyChat, () => {
        dispatch(resetDailyChat(null));
    });
};
