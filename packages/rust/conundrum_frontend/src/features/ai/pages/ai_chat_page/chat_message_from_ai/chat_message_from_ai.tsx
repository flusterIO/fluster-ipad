import { StreamingMarkdown } from "#/streaming/markdown/streaming_markdown";
import { type AIMessage } from "@/codegen/bindings";
import React, { type ReactNode } from "react";
import { ChatMessageContainer } from "../chat_message_from_user/chat_message_container";
import { type PartialAgentMessage } from "#/ai/state/hooks/use_formatted_chat_history";

interface ChatMessageFromAIProps {
    item: AIMessage | PartialAgentMessage;
    index: number;
    isLast: boolean;
    animate?: boolean;
}

export const ChatMessageFromAgent = ({
    item,
    index,
    isLast,
    animate,
}: ChatMessageFromAIProps): ReactNode => {
    return (
        <ChatMessageContainer
            isLast={isLast}
            className="w-full h-fit rounded"
            index={index}
            animate={animate}
        >
            <StreamingMarkdown activelyStreaming={false}>
                {item.body}
            </StreamingMarkdown>
        </ChatMessageContainer>
    );
};

ChatMessageFromAgent.displayName = "ChatMessageFromAI";
