import { type ChatMessageResultItem } from "#/database/db_utility_types/chat";
import React, { type ReactNode } from "react";

interface ChatMessageFromAIProps {
    item: ChatMessageResultItem;
}

export const ChatMessageFromAI = ({
    item,
}: ChatMessageFromAIProps): ReactNode => {
    return <div>{item.body}</div>;
};

ChatMessageFromAI.displayName = "ChatMessageFromAI";
