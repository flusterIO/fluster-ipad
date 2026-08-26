import { StreamingMarkdown } from "#/streaming/markdown/streaming_markdown";
import { type ReasoningBlock } from "@/codegen/bindings";
import React, { type ReactNode } from "react";
import { ChatMessageContainer } from "../chat_message_from_user/chat_message_container";
import { type ReasoningBlockPartial } from "#/ai/state/hooks/use_formatted_chat_history";

interface ReasoningTextComponentProps {
    item: ReasoningBlock | ReasoningBlockPartial;
    index: number | "stream";
    isLast: boolean;
    animate: boolean;
}

export const ReasoningContent = ({
    index,
    isLast,
    animate,
    children,
}: Omit<ReasoningTextComponentProps, "item"> & {
    children: string;
}): ReactNode => {
    return (
        <ChatMessageContainer
            animate={animate}
            isLast={isLast}
            className="text-sm w-full"
            index={index}
        >
            <StreamingMarkdown
                activelyStreaming={false}
                className="w-full bg-card rounded p-4 text-sm *:text-foreground/80!"
            >
                {children}
            </StreamingMarkdown>
        </ChatMessageContainer>
    );
};

export const ReasoningTextComponent = ({
    item,
    ...props
}: ReasoningTextComponentProps): ReactNode => {
    return <ReasoningContent {...props}>{item.content}</ReasoningContent>;
};

ReasoningTextComponent.displayName = "ReasoningTextComponent";
