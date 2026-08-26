import React, { type ReactNode } from "react";
import { isEmptyChatResponse } from "../use_chat";
import { useChatPageContext } from "../chat_page_context/chat_page_context";
import { ChatMessageContent } from "../chat_message_from_ai/chat_message_from_ai";
import { ReasoningContent } from "../chat_message_from_ai/reasoning_text_from_ai";

export const CurrentlyStreamingMessage = (): ReactNode => {
    const { response: data } = useChatPageContext();
    if (!data || isEmptyChatResponse(data)) {
        return null;
    }
    const { reasoning, response } = data;
    return (
        <>
            {reasoning.map((r) => {
                return (
                    <ReasoningContent animate={true} index={"stream"} isLast={false}>
                        {r}
                    </ReasoningContent>
                );
            })}
            <ChatMessageContent index={"stream"} animate={true} isLast={false}>
                {response}
            </ChatMessageContent>
        </>
    );
};

CurrentlyStreamingMessage.displayName = "CurrentlyStreamingMessage";
