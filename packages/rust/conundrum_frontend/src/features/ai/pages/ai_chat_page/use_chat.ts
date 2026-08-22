import { type ChatClientData } from "#/database/db_utility_types/chat";
import { useSelector } from "react-redux";
import { v4 } from "uuid";
import { getServerPort } from "@/app/rspc_client";
import consola from "consola";
import { useCallback, useEffect, useRef, useState } from "react";
import { useSearchParams } from "react-router";

import {
    type ChatEvent,
    type UserMessageInput,
} from "@conundrum/ts/codegen-typeshare";
import { type ToolExecution } from "@/codegen/bindings";
import { type AppState } from "@/state/initial_state";
import { useLogger } from "#/logging/state/hooks/use_logger";

/**
 * Used by the timeout function in case the provider doesn't send a 'final' request so the data can be sent back to the server.
 */

export interface ChatSearchParams {
    agent: string;
    /**
     * If undefined, a new conversation will be created.
     */
    convo?: string;
    /**
     * Set internally by the Conundrum useChat hoook when a user scrolls to the top.
     */
    page?: number;
}

export interface ChatData extends Pick<
    ChatClientData,
    "reasoning" | "response" | "tokens" | "tool_calls" | "system_prompt"
> {
    reasoning: string[];
    response: string;
    reasoningSummary?: string;
    toolCalls: ToolExecution[];
    system_prompt: "";
    tokens: {
        total: number;
        incoming: number;
        outgoing: number;
    };
}

export const isEmptyChatResponse = (cd: ChatData): boolean => {
    return (
        !cd.reasoning.length &&
        !cd.response.length &&
        !cd.toolCalls.length &&
        !cd.reasoningSummary?.length
    );
};

export const getEmptyChatData = (): ChatData => {
    return {
        reasoning: [],
        response: "",
        reasoningSummary: "",
        toolCalls: [],
        system_prompt: "",
        tool_calls: [],
        tokens: {
            total: 0,
            incoming: 0,
            outgoing: 0,
        },
    };
};

// export const useChat = () => {

//     return {
//         ref: container,
//         response,
//         connected,
//         sendMessage,
//         activelyStreaming,
//     };
// };
