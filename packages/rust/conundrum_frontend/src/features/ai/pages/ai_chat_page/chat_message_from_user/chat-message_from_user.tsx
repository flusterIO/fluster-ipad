import { StreamingMarkdown } from "#/streaming/markdown/streaming_markdown";
import { type UserMessage } from "@/codegen/bindings";
import React, { type ReactNode } from "react";
import { motion } from "framer-motion";
import { ChatMessageContainer } from "./chat_message_container";

const StreamingMarkdownMotion = motion.create(StreamingMarkdown);

interface ChatMessageFromUserProps {
    item: UserMessage;
    index: number;
    isLast: boolean;
}

export const ChatMessageFromUser = ({
    item,
    index,
    isLast,
}: ChatMessageFromUserProps): ReactNode => {
    return (
        <ChatMessageContainer
            className="bg-primary p-2 rounded max-w-[80%] w-fit"
            index={index}
            isLast={isLast}
            ctime={item.ctime}
        >
            <StreamingMarkdownMotion activelyStreaming={false}>
                {item.body}
            </StreamingMarkdownMotion>
        </ChatMessageContainer>
    );
};

ChatMessageFromUser.displayName = "ChatMessageFromUser";
