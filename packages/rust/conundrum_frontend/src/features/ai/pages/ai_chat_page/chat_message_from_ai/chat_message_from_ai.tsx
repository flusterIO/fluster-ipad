import React, { type ReactNode } from "react";

interface ChatMessageFromAIProps {
    content: string;
}

export const ChatMessageFromAI = ({
    content,
}: ChatMessageFromAIProps): ReactNode => {
    return <div>{content}</div>;
};

ChatMessageFromAI.displayName = "ChatMessageFromAI";
