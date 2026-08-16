import {
    PromptInput,
    PromptInputBody,
    PromptInputButton,
    PromptInputFooter,
    PromptInputSubmit,
    PromptInputTextarea,
    PromptInputTools,
} from "@/components/ai_elements/prompt_input";
import { randomFromArray } from "@/utils/array_utils";
import { MicIcon, PaperclipIcon } from "lucide-react";
import { motion } from "framer-motion";
import React, { useMemo, useRef, useState, type ReactNode } from "react";
import { useChat } from "./use_chat";
import { ChatMessageFromUser } from "./chat_message_from_user/chat-message_from_user";
import { ChatMessageFromAI } from "./chat_message_from_ai/chat_message_from_ai";
import { CurrentlyStreamingMessage } from "./currently_streaming_message/currently_streaming_message";
import { cn } from "@/utils/shad_utils";

export const GeneralAIChatPage = (): ReactNode => {
    const placeholder = useMemo(() => {
        return randomFromArray([
            "How can I help?",
            "What are we working on today?",
            "What would you like to know?",
            "Anything I can help with?",
            "I'm here for whatever you need",
            "Let's get to work...",
            "Let's change the world...",
        ]);
    }, []);
    const { sendMessage, messages, ref, response, activelyStreaming } = useChat();
    console.log("response: ", response);
    return (
        <div className="w-full h-screen max-h-screen px-4">
            <div className="mx-auto w-270 max-w-[calc(100%-4rem)] max-h-screen min-h-screen flex flex-col justify-between items-center">
                <motion.div
                    ref={ref}
                    className={
                        "grow overflow-x-hidden overflow-y-auto w-[calc(100%+0.5rem)] translate-x-1 min-scrollbar mb-2 flex flex-col justify-end items-center"
                    }
                >
                    {messages.map((m) => {
                        switch (m.sender) {
                            case "user":
                                return <ChatMessageFromUser item={m} key={m.id} />;
                            case "ai":
                                return <ChatMessageFromAI item={m} key={m.id} />;
                            case "system_prompt":
                                return null;
                        }
                    })}
                    <CurrentlyStreamingMessage
                        {...response}
                        activelyStreaming={activelyStreaming}
                    />
                </motion.div>
                <PromptInput
                    onSubmit={(val) => {
                        sendMessage(val.text);
                    }}
                    className="mb-4 mx-4 w-full"
                >
                    <PromptInputBody>
                        <PromptInputTextarea placeholder={placeholder} />
                    </PromptInputBody>
                    <PromptInputFooter>
                        <PromptInputTools>
                            <PromptInputButton>
                                <PaperclipIcon size={16} />
                            </PromptInputButton>
                            <PromptInputButton tooltip={"Voice Input"}>
                                <MicIcon size={16} />
                            </PromptInputButton>
                        </PromptInputTools>
                        <PromptInputSubmit />
                    </PromptInputFooter>
                </PromptInput>
            </div>
        </div>
    );
};

GeneralAIChatPage.displayName = "GeneralAIChatPage";
