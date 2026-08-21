import { StreamingMarkdown } from "#/streaming/markdown/streaming_markdown";
import { type AIMessage } from "@/codegen/bindings";
import React, { type ReactNode } from "react";
import { ChatMessageContainer } from "../chat_message_from_user/chat_message_container";

interface ChatMessageFromAIProps {
    item: AIMessage;
    index: number;
}

export const ChatMessageFromAgent = ({
    item,
    index,
}: ChatMessageFromAIProps): ReactNode => {
    return (
        <ChatMessageContainer className="w-full h-fit rounded" index={index}>
            <StreamingMarkdown activelyStreaming={false}>
                {item.body}
            </StreamingMarkdown>
        </ChatMessageContainer>
    );
};

ChatMessageFromAgent.displayName = "ChatMessageFromAI";
