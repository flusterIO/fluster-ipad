import { type ChatHistoryResponse } from "#/database/db_utility_types/chat";
import {
    type ToolExecution,
    type AIMessage,
    type ReasoningBlock,
    type SystemPromptMessage,
    type UserMessage,
} from "@/codegen/bindings";
import { type UserMessageInput } from "@conundrum/ts/codegen-typeshare";
import { useMemo } from "react";

export type FormattedChatHistoryItem =
    | {
        type: "user-message";
        data: UserMessage;
    }
    | {
        type: "system-message";
        data: SystemPromptMessage;
    }
    | {
        type: "agent-message";
        data: AIMessage;
    }
    | {
        type: "reasoning-block";
        data: ReasoningBlock;
    }
    | {
        type: "tool-execution";
        data: ToolExecution;
    }
    | {
        type: "user-partial";
        data: UserMessageInput & { ctime: Date };
    };

export const useFormattedChatHistory = (
    chatHistory: ChatHistoryResponse | null,
) => {
    return useMemo((): FormattedChatHistoryItem[] => {
        if (!chatHistory) {
            return [] as FormattedChatHistoryItem[];
        }
        return (
            [
                ...chatHistory[0].map((k) => {
                    return {
                        type: "user-message",
                        data: k,
                    } satisfies FormattedChatHistoryItem;
                }),
                ...chatHistory[1].map((k) => {
                    return {
                        type: "system-message",
                        data: k,
                    } satisfies FormattedChatHistoryItem;
                }),
                ...chatHistory[2].map((k) => {
                    return {
                        type: "agent-message",
                        data: k,
                    } satisfies FormattedChatHistoryItem;
                }),
                ...chatHistory[3].map((k) => {
                    return {
                        type: "reasoning-block",
                        data: k,
                    } satisfies FormattedChatHistoryItem;
                }),
                ...chatHistory[4].map((k) => {
                    return {
                        type: "tool-execution",
                        data: k,
                    } satisfies FormattedChatHistoryItem;
                }),
            ] satisfies FormattedChatHistoryItem[]
        ).sort((a, b) => {
            return new Date(a.data.ctime).valueOf() < new Date(b.data.ctime).valueOf()
                ? -1
                : 1;
        });
    }, [chatHistory]);
};
