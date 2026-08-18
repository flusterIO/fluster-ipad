import React, { type ReactNode } from "react";
import { useChat } from "../pages/ai_chat_page/use_chat";

interface MemoizedPromptOutputProps {
    prompt: string;
}

export const MemoizedPromptOutput = ({
    prompt,
}: MemoizedPromptOutputProps): ReactNode => {
    const { sendMessage, input } = useChat();
    return <div></div>;
};

MemoizedPromptOutput.displayName = "MemoizedPromptOutput";
