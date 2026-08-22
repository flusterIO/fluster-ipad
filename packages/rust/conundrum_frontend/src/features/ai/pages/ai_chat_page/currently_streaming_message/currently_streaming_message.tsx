import React, { type ReactNode } from "react";
import { isEmptyChatResponse } from "../use_chat";
import { StreamingMarkdown } from "#/streaming/markdown/streaming_markdown";
import { motion } from "framer-motion";
import { useChatPageContext } from "../chat_page_context/chat_page_context";

export const CurrentlyStreamingMessage = (): ReactNode => {
    const { response: data, thinking } = useChatPageContext();
    if (!data || isEmptyChatResponse(data)) {
        return null;
    }
    const { reasoning, response } = data;
    return (
        <motion.div
            className="w-full flex flex-col justify-start items-start rounded p-4 origin-bottom my-3"
            initial={{
                scale: 0,
                opacity: 0,
            }}
            animate={{
                scale: 1,
                opacity: 1,
            }}
        >
            {reasoning.map((r) => {
                return (
                    <StreamingMarkdown
                        activelyStreaming={thinking}
                        className="w-full bg-card rounded p-4 text-sm *:text-foreground/80!"
                    >
                        {r}
                    </StreamingMarkdown>
                );
            })}
            <StreamingMarkdown className="max-w-full" activelyStreaming={thinking}>
                {response}
            </StreamingMarkdown>
        </motion.div>
    );
};

CurrentlyStreamingMessage.displayName = "CurrentlyStreamingMessage";
