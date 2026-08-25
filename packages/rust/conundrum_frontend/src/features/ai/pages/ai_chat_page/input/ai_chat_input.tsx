import {
    PromptInputBody,
    PromptInputTextarea,
    PromptInputFooter,
    PromptInputTools,
    PromptInputSubmit,
    PromptInput,
} from "@/components/ai_elements/prompt_input";
import React, { useMemo, type ReactNode } from "react";
import { motion } from "framer-motion";
import { useChatPageContext } from "../chat_page_context/chat_page_context";
import { randomFromArray } from "@/utils/array_utils";
import { MicSelector } from "./buttons/mic_select";
import { useMessageSender } from "../chat_page_context/use_message_sender";
import { AgentSelectInputButton } from "./buttons/agent_select";

const MotionInput = motion.create(PromptInput);

export const ChatInput = (): ReactNode => {
    const sendMessage = useMessageSender();
    const { thinking } = useChatPageContext();

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
    return (
        <MotionInput
            onSubmit={(val) => {
                if (val.text.trim().length) {
                    sendMessage(val.text);
                }
            }}
            className="mb-4 mx-4 w-full"
            initial={{
                y: "100%",
                opacity: 0,
            }}
            animate={{
                y: 0,
                opacity: 1,
            }}
            exit={{
                y: "100%",
                opacity: 0,
            }}
        >
            <PromptInputBody>
                <PromptInputTextarea placeholder={placeholder} />
            </PromptInputBody>
            <PromptInputFooter>
                <PromptInputTools>
                    <AgentSelectInputButton />
                    <MicSelector />
                    {thinking ? (
                        <div className="ml-4 w-2 h-2 rounded-full bg-primary animate-ping" />
                    ) : null}
                </PromptInputTools>
                <PromptInputSubmit />
            </PromptInputFooter>
        </MotionInput>
    );
};

ChatInput.displayName = "ChatInput";
