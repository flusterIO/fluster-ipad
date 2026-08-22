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
import { MicIcon, PersonStandingIcon, SearchIcon } from "lucide-react";
import { motion } from "framer-motion";
import React, { useMemo, useState, type ReactNode } from "react";
import { isEmptyChatResponse, useChat } from "./use_chat";
import { CurrentlyStreamingMessage } from "./currently_streaming_message/currently_streaming_message";
import { ChatSideSheet } from "./chat_selection_sheet/chat_selection_sheet";
import { Button } from "@/components/shad/button";
import { EmptyChat } from "./empty_chat/empty_chat";
import { ChatContent } from "./chat_content";
import { useSearchParams } from "react-router";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { cn } from "@/utils/shad_utils";
import {
    MicSelector,
    MicSelectorTrigger,
} from "@/components/ai_elements/mic_selector";
import { OllamaModelSelect } from "#/ai/components/ollama_model_select/ollama_model_select";

const MotionButton = motion.create(Button);
const MotionInput = motion.create(PromptInput);

interface EventProps {
    has_messages: boolean;
}

declare global {
    interface WindowEventMap {
        "set-ai-message-count": CustomEvent<EventProps>;
        "set-chat-entrance-settled": CustomEvent<{ chat_id: string }>;
    }
}

const GeneralAIChatPageContent = (): ReactNode => {
    const [sp] = useSearchParams();
    const convo_id = sp.get("convo");
    if (!convo_id) {
        return null;
    }
    const setHasMessages = (hasMessages: boolean): void => {
        window.dispatchEvent(
            new CustomEvent("set-ai-message-count", {
                detail: {
                    has_messages: hasMessages,
                },
            }),
        );
    };
    return <ChatContent convo_id={convo_id} setHasMessages={setHasMessages} />;
};

export const GeneralAIChatPageInner = ({
    children,
}: {
    children: ReactNode;
}): ReactNode => {
    const [sheetOpen, setSheetOpen] = useState(false);
    const [hasMessages, setHasMessages] = useState(false);
    useEventListener("set-ai-message-count", (e) => {
        setHasMessages(e.detail.has_messages);
    });
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
    const { sendMessage, ref, response, activelyStreaming } = useChat();
    const showingEmptyChat = useMemo(() => {
        return isEmptyChatResponse(response) && !hasMessages;
    }, [response, hasMessages]);
    console.log("showingEmptyChahowing: ", showingEmptyChat);
    return (
        <div className="w-full h-screen max-h-screen px-4">
            <motion.div
                className="absolute top-4 right-4 z-10 opacity-90 hover:opacity-100 hover:bg-secondary rounded cursor-pointer"
                whileHover={{
                    backgroundColor: "hsl(var(--secondary))",
                }}
            >
                <MotionButton
                    transitionAll={false}
                    key="general-ai-search"
                    size={hasMessages ? "icon-xs" : "icon-lg"}
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
            </motion.div>
            <div
                className={cn(
                    "@container/chat mx-auto w-270 max-w-[calc(100%-2rem)] max-h-screen min-h-screen flex flex-col justify-between items-center",
                )}
            >
                <motion.div
                    ref={ref}
                    className={cn(
                        "grow overflow-x-hidden overflow-y-auto w-[calc(100%+0.5rem)] translate-x-1 no-scrollbar flex flex-col justify-end items-center pb-4",
                        showingEmptyChat && "showing-empty-chat",
                    )}
                >
                    {children}
                    {showingEmptyChat ? (
                        <EmptyChat />
                    ) : (
                        <CurrentlyStreamingMessage
                            {...response}
                            activelyStreaming={activelyStreaming}
                        />
                    )}
                </motion.div>
                <div className="relative h-0! w-full overflow-visible">
                    <div
                        className="w-full h-4 absolute bottom-0"
                        style={{
                            background:
                                "linear-gradient(hsl(var(--background)/0.1), hsl(var(--background)))",
                        }}
                    />
                </div>
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
                                <OllamaModelSelect>
                                    <PersonStandingIcon />
                                </OllamaModelSelect>
                            </PromptInputButton>
                            <PromptInputButton tooltip={"Voice Input"}>
                                <MicSelector>
                                    <MicSelectorTrigger>
                                        <MicIcon size={16} />
                                    </MicSelectorTrigger>
                                </MicSelector>
                            </PromptInputButton>
                            {activelyStreaming ? (
                                <div className="w-2 h-2 rounded-full bg-primary animate-ping" />
                            ) : null}
                        </PromptInputTools>
                        <PromptInputSubmit />
                    </PromptInputFooter>
                </MotionInput>
            </div>
            <ChatSideSheet
                open={sheetOpen}
                close={() => {
                    setSheetOpen(false);
                }}
            />
        </div>
    );
};

GeneralAIChatPageInner.displayName = "GeneralAIChatPage";

/**
 * All of this to get around the content component rendering when the stream output changes.
 */
export const GeneralAIChatPage = () => {
    return (
        <GeneralAIChatPageInner>
            <GeneralAIChatPageContent />
        </GeneralAIChatPageInner>
    );
};
