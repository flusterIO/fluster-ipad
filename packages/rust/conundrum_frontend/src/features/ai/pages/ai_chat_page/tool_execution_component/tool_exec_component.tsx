import { type ToolExecution } from "@/codegen/bindings";
import React, { type ReactNode } from "react";
import { ChatMessageContainer } from "../chat_message_from_user/chat_message_container";

interface ToolExecComponentProps {
    item: ToolExecution;
    isLast: boolean;
    index: number;
}

export const ToolExecComponent = ({
    item,
    isLast,
    index,
}: ToolExecComponentProps): ReactNode => {
    return (
        <ChatMessageContainer
            className="w-full h-fit"
            index={index}
            isLast={isLast}
        >
            <div className="w-full h-fit flex flex-row justify-center items-center">
                <div>{item.tool_name}</div>
            </div>
        </ChatMessageContainer>
    );
};

ToolExecComponent.displayName = "ToolExecComponent";
