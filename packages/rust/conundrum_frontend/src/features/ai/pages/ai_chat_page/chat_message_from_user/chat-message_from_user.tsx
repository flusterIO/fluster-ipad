import { StreamingMarkdown } from "#/streaming/markdown/streaming_markdown";
import { type UserMessage } from "@/codegen/bindings";
import React, { type ReactNode } from "react";

interface ChatMessageFromUserProps {
    item: UserMessage;
}

export const ChatMessageFromUser = ({
    item,
}: ChatMessageFromUserProps): ReactNode => {
    return (
        <StreamingMarkdown activelyStreaming={false}>{item.body}</StreamingMarkdown>
    );
};

ChatMessageFromUser.displayName = "ChatMessageFromUser";
