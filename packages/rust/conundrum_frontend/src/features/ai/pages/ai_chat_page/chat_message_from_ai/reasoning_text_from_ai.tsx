import { StreamingMarkdown } from "#/streaming/markdown/streaming_markdown";
import { type ReasoningBlock } from "@/codegen/bindings";
import React, { type ReactNode } from "react";
import { ChatMessageContainer } from "../chat_message_from_user/chat_message_container";

interface ReasoningTextComponentProps {
    item: ReasoningBlock;
    index: number;
    isLast: boolean;
}

export const ReasoningTextComponent = ({
    item,
    index,
    isLast,
}: ReasoningTextComponentProps): ReactNode => {
    return (
        <ChatMessageContainer
            isLast={isLast}
            className="text-sm font-serif"
            index={index}
        >
            <StreamingMarkdown activelyStreaming={false}>
                {item.content}
            </StreamingMarkdown>
        </ChatMessageContainer>
    );
};

ReasoningTextComponent.displayName = "ReasoningTextComponent";
