import React, { type ReactNode } from "react";

interface ChatMessageFromUserProps {
    content: string;
}

export const ChatMessageFromUser = ({
    content,
}: ChatMessageFromUserProps): ReactNode => {
    return <div>{content}</div>;
};

ChatMessageFromUser.displayName = "ChatMessageFromUser";
