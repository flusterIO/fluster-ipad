import { useFormattedChatHistory } from "#/ai/state/hooks/use_formatted_chat_history";
import { rspc } from "@/app/rspc_client";
import React, { useEffect, useState, type ReactNode } from "react";
import { ChatMessageFromUser } from "./chat_message_from_user/chat-message_from_user";
import { ChatMessageFromAgent } from "./chat_message_from_ai/chat_message_from_ai";
import { ReasoningTextComponent } from "./chat_message_from_ai/reasoning_text_from_ai";
import { ToolExecComponent } from "./tool_execution_component/tool_exec_component";

interface ChatContentProps {
    convo_id: string;
    setHasMessages: (hasLength: boolean) => void;
}

export const ChatContent = ({
    convo_id,
    setHasMessages,
}: ChatContentProps): ReactNode => {
    const [page, setPage] = useState(1);
    const { data: chatHistory } = rspc.useQuery([
        "agent.load_chat_history",
        {
            convo_id,
            max_count: page * 10,
        },
    ]);
    console.log("chatHistory: ", chatHistory);
    const data = useFormattedChatHistory(chatHistory ?? null);

    useEffect(() => {
        setHasMessages(Boolean(data.length));
    }, [data]);
    return (
        <div className="w-full h-fit flex flex-col justify-end items-end gap-y-4">
            {data.map((d) => {
                if (d.type === "user-message") {
                    return <ChatMessageFromUser item={d.data} key={d.data.id} />;
                }
                if (d.type === "agent-message") {
                    return <ChatMessageFromAgent item={d.data} key={d.data.id} />;
                }
                if (d.type === "reasoning-block") {
                    return <ReasoningTextComponent item={d.data} key={d.data.id} />;
                }
                if (d.type === "tool-execution") {
                    return <ToolExecComponent item={d.data} key={d.data.id} />;
                }
                return null;
            })}
        </div>
    );
};

ChatContent.displayName = "ChatContent";
