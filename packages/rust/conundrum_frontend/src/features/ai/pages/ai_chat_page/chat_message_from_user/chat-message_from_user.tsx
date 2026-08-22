import { StreamingMarkdown } from "#/streaming/markdown/streaming_markdown";
import { type UserMessage } from "@/codegen/bindings";
import React, { type ReactNode } from "react";
import { motion } from "framer-motion";
import { ChatMessageContainer } from "./chat_message_container";
import { type UserMessageInput } from "@conundrum/ts/codegen-typeshare";

const StreamingMarkdownMotion = motion.create(StreamingMarkdown);

interface ChatMessageFromUserProps {
    item: UserMessage | (UserMessageInput & { ctime: Date });
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
            className="bg-primary p-2 rounded"
            containerClasses="max-w-[80%] w-fit"
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
