import { type ChatMessageResultItem } from "#/database/db_utility_types/chat";
import React, { type ReactNode } from "react";

interface ChatMessageFromUserProps {
    item: ChatMessageResultItem;
}

export const ChatMessageFromUser = ({
    item,
}: ChatMessageFromUserProps): ReactNode => {
    return <div className="text-foreground">{item.body}</div>;
};

ChatMessageFromUser.displayName = "ChatMessageFromUser";
