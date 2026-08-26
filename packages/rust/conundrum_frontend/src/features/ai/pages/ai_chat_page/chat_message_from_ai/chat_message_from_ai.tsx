import { StreamingMarkdown } from "#/streaming/markdown/streaming_markdown";
import { type AIMessage } from "@/codegen/bindings";
import React, { type ReactNode } from "react";
import { ChatMessageContainer } from "../chat_message_from_user/chat_message_container";
import { type PartialAgentMessage } from "#/ai/state/hooks/use_formatted_chat_history";

interface ChatMessageFromAIProps {
    item: AIMessage | PartialAgentMessage;
    index: number | "stream";
    isLast: boolean;
    animate?: boolean;
}

export const ChatMessageContent = ({
    children,
    index,
    isLast,
    animate,
}: Omit<ChatMessageFromAIProps, "item"> & { children: string }): ReactNode => {
    return (
        <ChatMessageContainer
            isLast={isLast}
            className="w-full h-fit rounded mb-4"
            index={index}
            animate={animate}
        >
            <StreamingMarkdown
                className="w-full [&>p]:w-full"
                activelyStreaming={false}
            >
                {children}
            </StreamingMarkdown>
        </ChatMessageContainer>
    );
};

export const ChatMessageFromAgent = ({
    item,
    ...props
}: ChatMessageFromAIProps): ReactNode => {
    return <ChatMessageContent {...props}>{item.body}</ChatMessageContent>;
};

ChatMessageFromAgent.displayName = "ChatMessageFromAI";
