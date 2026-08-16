import React, { type ReactNode } from "react";
import { type ChatData } from "../use_chat";
import { StreamingMarkdown } from "#/streaming/markdown/streaming_markdown";
import { motion } from "framer-motion";

interface CurrentlyStreamingMessageProps extends ChatData {
    activelyStreaming: boolean;
}

export const CurrentlyStreamingMessage = ({
    reasoning,
    response,
    toolCalls,
    activelyStreaming,
}: CurrentlyStreamingMessageProps): ReactNode => {
    if (!reasoning.length && !response.length && !toolCalls.length) {
        return null;
    }
    return (
        <motion.div
            className="w-full flex flex-col justify-start items-start rounded p-4 border origin-bottom"
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
                        activelyStreaming={activelyStreaming}
                        className="w-full bg-card rounded p-4 text-sm *:text-foreground/80!"
                    >
                        {r}
                    </StreamingMarkdown>
                );
            })}
            <StreamingMarkdown activelyStreaming={activelyStreaming}>
                {response}
            </StreamingMarkdown>
        </motion.div>
    );
};

CurrentlyStreamingMessage.displayName = "CurrentlyStreamingMessage";
