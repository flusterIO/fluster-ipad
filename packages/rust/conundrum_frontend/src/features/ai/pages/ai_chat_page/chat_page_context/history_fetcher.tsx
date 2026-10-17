import { useFormattedChatHistory } from "#/ai/state/hooks/use_formatted_chat_history";
import { rspc } from "@/app/rspc_client";
import React, { useEffect, type ReactNode } from "react";
import { useChatPageDispatch } from "./chat_page_context";
import { useSearchParams } from "react-router";
import { useObserveChatLocationState } from "./use_observe_chat_location_state";

interface HistoryFetcherProps {
    convo_id: string;
    page?: number;
}

const HistoryFetcherInner = ({
    convo_id,
    page = 1,
}: HistoryFetcherProps): ReactNode => {
    const { data: chatHistory } = rspc.useQuery(
        [
            "agent.load_chat_history",
            {
                convo_id,
                max_count: page * 10,
            },
        ],
        {
            refetchOnWindowFocus: false,
            refetchOnReconnect: true,
            refetchOnMount: true,
        },
    );
    const { data: conversations } = rspc.useQuery([
        "crud.chat_conversation.get_by_predicate",
        {
            predicate: `id = "${convo_id}"`,
            pagination: {
                page: 1,
                per_page: 1,
            },
        },
    ]);
    const dispatch = useChatPageDispatch();
    const data = useFormattedChatHistory(chatHistory ?? null);
    useEffect(() => {
        dispatch({
            type: "set-messages",
            payload: data,
        });
    }, [data]);
    useEffect(() => {
        dispatch({
            type: "set-selected-model",
            payload: conversations?.[0]?.selected_model ?? null,
        });
    }, [conversations, convo_id]);
    return null;
};

export const HistoryFetcher = ({ page }: { page?: number }): ReactNode => {
    useObserveChatLocationState();
    const [sp] = useSearchParams();
    const convo = sp.get("convo");
    if (convo) {
        return <HistoryFetcherInner convo_id={convo} page={page} />;
    } else {
        return null;
    }
};

HistoryFetcher.displayName = "HistoryFetcher";
