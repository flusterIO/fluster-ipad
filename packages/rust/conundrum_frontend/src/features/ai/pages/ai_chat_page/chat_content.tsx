import React, { type ReactNode } from "react";
import { ChatMessageFromUser } from "./chat_message_from_user/chat-message_from_user";
import { ChatMessageFromAgent } from "./chat_message_from_ai/chat_message_from_ai";
import { ReasoningTextComponent } from "./chat_message_from_ai/reasoning_text_from_ai";
import { ToolExecComponent } from "./tool_execution_component/tool_exec_component";
import { AnimatePresence } from "framer-motion";
import { type UserMessage } from "@/codegen/bindings";
import { useChatPageContext } from "./chat_page_context/chat_page_context";
import { motion } from "framer-motion";

interface EventProps {
    message: UserMessage;
}

declare global {
    interface WindowEventMap {
        "append-user-message": CustomEvent<EventProps>;
    }
}

export const ChatContent = (): ReactNode => {
    const { messages } = useChatPageContext();
    return (
        </* className={ */
      /*     "w-full h-fit flex flex-col justify-end items-end gap-y-4 px-2 mt-4 chat-content" */
      /* } */
      /* exit={{ */
      /*     opacity: 0, */
      /* }} */>
            <AnimatePresence presenceAffectsLayout>
                {messages.map((d, i) => {
                    if (d.type === "user-message" || d.type === "user-partial") {
                        return (
                            <ChatMessageFromUser
                                isLast={i === messages.length - 1}
                                index={i}
                                item={d.data}
                                key={"id" in d.data ? d.data.id : `${d.data.body}${i}`}
                            />
                        );
                    }
                    if (d.type === "agent-message" || d.type === "agent-partial") {
                        return (
                            <ChatMessageFromAgent
                                isLast={i === messages.length - 1}
                                index={i}
                                item={d.data}
                                key={"id" in d.data ? d.data.id : `${d.data.body}${i}`}
                                animate={d.type === "agent-message"}
                            />
                        );
                    }
                    if (d.type === "reasoning-block" || d.type === "reasoning-partial") {
                        return (
                            <ReasoningTextComponent
                                isLast={i === messages.length - 1}
                                index={i}
                                item={d.data}
                                key={"id" in d.data ? d.data.id : `${d.data.content}${i}`}
                                animate={d.type === "reasoning-block"}
                            />
                        );
                    }
                    if (d.type === "tool-execution") {
                        return (
                            <ToolExecComponent
                                index={i}
                                isLast={i === messages.length - 1}
                                item={d.data}
                                key={d.data.id}
                            />
                        );
                    }
                    return null;
                })}
            </AnimatePresence>
        </>
    );
};

ChatContent.displayName = "ChatContent";
