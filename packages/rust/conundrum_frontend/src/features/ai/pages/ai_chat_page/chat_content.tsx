import { useFormattedChatHistory } from "#/ai/state/hooks/use_formatted_chat_history";
import { rspc } from "@/app/rspc_client";
import React, { useEffect, useState, type ReactNode } from "react";
import { ChatMessageFromUser } from "./chat_message_from_user/chat-message_from_user";
import { ChatMessageFromAgent } from "./chat_message_from_ai/chat_message_from_ai";
import { ReasoningTextComponent } from "./chat_message_from_ai/reasoning_text_from_ai";
import { ToolExecComponent } from "./tool_execution_component/tool_exec_component";
import { AnimatePresence } from "framer-motion";
import { type UserMessage } from "@/codegen/bindings";
import { useEventListener } from "@/state/hooks/use_event_listener";
import consola from "consola";

interface ChatContentProps {
    convo_id: string;
    setHasMessages: (hasLength: boolean) => void;
}

interface EventProps {
    message: UserMessage;
}

declare global {
    interface WindowEventMap {
        "append-user-message": CustomEvent<EventProps>;
    }
}

export const ChatContent = ({
    convo_id,
    setHasMessages,
}: ChatContentProps): ReactNode => {
    const [page, setPage] = useState(1);
    const { data: chatHistory, refetch } = rspc.useQuery(
        [
            "agent.load_chat_history",
            {
                convo_id,
                max_count: page * 10,
            },
        ],
        {
            refetchOnWindowFocus: true,
            refetchOnReconnect: true,
            refetchOnMount: true,
        },
    );
    const data = useFormattedChatHistory(chatHistory ?? null);

    useEventListener("append-user-message", (e) => {
        if (e.detail.message.convo_id === convo_id) {
            refetch().catch((err: unknown) => {
                consola.error("Error: ", err);
            });
        }
    });

    useEffect(() => {
        setHasMessages(Boolean(data.length));
    }, [data]);
    return (
        <div
            className={
                "w-full h-fit flex flex-col justify-end items-end gap-y-4 px-2 mt-4 chat-content"
            }
        >
            <AnimatePresence presenceAffectsLayout>
                {data.map((d, i) => {
                    if (d.type === "user-message") {
                        return (
                            <ChatMessageFromUser
                                isLast={i === data.length - 1}
                                index={i}
                                item={d.data}
                                key={d.data.id}
                            />
                        );
                    }
                    if (d.type === "agent-message") {
                        return (
                            <ChatMessageFromAgent
                                isLast={i === data.length - 1}
                                index={i}
                                item={d.data}
                                key={d.data.id}
                            />
                        );
                    }
                    if (d.type === "reasoning-block") {
                        return (
                            <ReasoningTextComponent
                                isLast={i === data.length - 1}
                                index={i}
                                item={d.data}
                                key={d.data.id}
                            />
                        );
                    }
                    if (d.type === "tool-execution") {
                        return (
                            <ToolExecComponent
                                index={i}
                                isLast={i === data.length - 1}
                                item={d.data}
                                key={d.data.id}
                            />
                        );
                    }
                    return null;
                })}
            </AnimatePresence>
        </div>
    );
};

ChatContent.displayName = "ChatContent";
