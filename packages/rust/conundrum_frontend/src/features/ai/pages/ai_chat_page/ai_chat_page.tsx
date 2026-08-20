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
import { MicIcon, PaperclipIcon, SearchIcon } from "lucide-react";
import { motion } from "framer-motion";
import React, { useMemo, useState, type ReactNode } from "react";
import { isEmptyChatResponse, useChat } from "./use_chat";
import { ChatMessageFromUser } from "./chat_message_from_user/chat-message_from_user";
import { ChatMessageFromAI } from "./chat_message_from_ai/chat_message_from_ai";
import { CurrentlyStreamingMessage } from "./currently_streaming_message/currently_streaming_message";
import { ChatSelectionSheet } from "./chat_selection_sheet/chat_selection_sheet";
import { Button } from "@/components/shad/button";
import { EmptyChat } from "./empty_chat/empty_chat";

const MotionButton = motion.create(Button);
const MotionInput = motion.create(PromptInput);

export const GeneralAIChatPage = (): ReactNode => {
    const [sheetOpen, setSheetOpen] = useState(false);
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
    return (
        <div className="w-full h-screen max-h-screen px-4">
            <div className="absolute top-4 right-4">
                <MotionButton
                    transitionAll={false}
                    key="general-ai-search"
                    size={messages.length ? "icon-xs" : "icon-lg"}
                    variant="secondary"
                    onClick={() => {
                        setSheetOpen(true);
                    }}
                    initial={"hide"}
                    variants={{
                        hide: {
                            x: 50,
                            opacity: 0,
                        },
                        small: {
                            x: 0,
                            opacity: 1,
                            width: 24,
                            height: 24,
                        },
                        large: {
                            x: 0,
                            opacity: 1,
                            width: 40,
                            height: 40,
                        },
                    }}
                    animate={isEmptyChatResponse(response) ? "large" : "small"}
                    exit={"hide"}
                >
                    <SearchIcon />
                </MotionButton>
            </div>
            <div className="@container/chat mx-auto w-270 max-w-[calc(100%-4rem)] max-h-screen min-h-screen flex flex-col justify-between items-center">
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
                    {isEmptyChatResponse(response) ? <EmptyChat /> : null}
                </motion.div>
                <MotionInput
                    onSubmit={(val) => {
                        sendMessage(val.text);
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
                            <PromptInputButton>
                                <PaperclipIcon size={16} />
                            </PromptInputButton>
                            <PromptInputButton tooltip={"Voice Input"}>
                                <MicIcon size={16} />
                            </PromptInputButton>
                            {activelyStreaming ? (
                                <div className="w-2 h-2 rounded-full bg-primary animate-ping" />
                            ) : null}
                        </PromptInputTools>
                        <PromptInputSubmit />
                    </PromptInputFooter>
                </MotionInput>
            </div>
            <ChatSelectionSheet
                open={sheetOpen}
                close={() => {
                    setSheetOpen(false);
                }}
            />
        </div>
    );
};

GeneralAIChatPage.displayName = "GeneralAIChatPage";
