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
import consola from "consola";
import { MicIcon, PaperclipIcon } from "lucide-react";
import React, { useMemo, type ReactNode } from "react";
import { useChat } from "./use_chat";
import { ChatMessageFromUser } from "./chat_message_from_user/chat-message_from_user";
import { ChatMessageFromAI } from "./chat_message_from_ai/chat_message_from_ai";
import { CurrentlyStreamingMessage } from "./currently_streaming_message/currently_streaming_message";

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
    const { sendMessage, value, setValue, messages, ref, response } = useChat();
    return (
        <div className="w-full h-screen flex flex-col justify-between items-center">
            <div
                ref={ref}
                className="grow w-full overflow-x-hidden overflow-y-auto min-scrollbar"
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
                <CurrentlyStreamingMessage {...response} />
            </div>
            <PromptInput
                onSubmit={(val) => {
                    sendMessage().catch((err: unknown) => {
                        consola.error("Error: ", err);
                    });
                }}
                className="mb-4 mx-4 w-270 max-w-[calc(100%-4rem)]"
            >
                <PromptInputBody>
                    <PromptInputTextarea
                        placeholder={placeholder}
                        value={value}
                        onChange={(e) => {
                            setValue(e.target.value);
                        }}
                    />
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
    );
};

GeneralAIChatPage.displayName = "GeneralAIChatPage";
