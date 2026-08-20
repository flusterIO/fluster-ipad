import { StreamingMarkdown } from "#/streaming/markdown/streaming_markdown";
import { type AIMessage } from "@/codegen/bindings";
import React, { type ReactNode } from "react";

interface ChatMessageFromAIProps {
    item: AIMessage;
}

export const ChatMessageFromAgent = ({
    item,
}: ChatMessageFromAIProps): ReactNode => {
    return (
        <StreamingMarkdown activelyStreaming={false}>{item.body}</StreamingMarkdown>
    );
};

ChatMessageFromAgent.displayName = "ChatMessageFromAI";
