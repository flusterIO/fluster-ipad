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
const STREAM_RESET_TIMEOUT = 2000;

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

export const useChat = () => {
    const container = useRef<HTMLDivElement>(null);
    const dailyChat = useSelector((state: AppState) => {
        return state.ai.dailyChat;
    });
    const [initialized, setInitialized] = useState(false);
    const [sp, setSp] = useSearchParams();
    const [activelyStreaming, setActivelyStreaming] = useState(false);
    const [response, setResponse] = useState<ChatData>(getEmptyChatData());
    const [connected, setConnected] = useState(false);
    const logger = useLogger();

    const page = sp.get("page") ?? "1";
    const agent_id = sp.get("agent");
    const conversation_id = sp.get("convo");

    useEffect(() => {
        if (!conversation_id) {
            sp.set("convo", dailyChat?.chat_id ?? v4());
            setSp(sp);
        }
    }, [conversation_id, dailyChat]);

    const socket = useRef<WebSocket | null>(null);

    const cleanupStream = useCallback(async () => {
        if (!activelyStreaming) {
            return;
        }
        if (!conversation_id) {
            consola.error("Failed to load conversation id.");
            setActivelyStreaming(false);
            return;
        }
        try {
            await logger({
                title: "Appended to Chat Context",
                message: `Your chat context was successfully updated for the chat with the id \`${conversation_id}\`.`,
                ai_description: `A user successfully appended content the chat history so that you can retrieve it later.`,
                purpose: "process-complete",
                severity: "success",
            });
            setActivelyStreaming(false);
        } catch (err: unknown) {
            consola.error("Error: ", err);
            setActivelyStreaming(false);
        }
    }, [conversation_id, agent_id, response, activelyStreaming]);

    useEffect(() => {
        if (initialized) {
            return;
        }
        const ws = new WebSocket(`ws://127.0.0.1:${getServerPort()}/api/ws`);

        socket.current = ws;

        ws.onopen = () => {
            consola.log("WebSocket connected");
            setConnected(true);
        };

        ws.onmessage = (event: MessageEvent<string>) => {
            try {
                const chatEvent = JSON.parse(event.data) as ChatEvent;

                const handleIndividualRequest = (req: ChatEvent) => {
                    if (req.type !== "done") {
                        setActivelyStreaming(true);
                    } else {
                        consola.log(`Tokens expended: `, req.content)
                        cleanupStream().catch((err: unknown) => {
                            consola.error("Error: {}", err);
                        });
                        return;
                    }
                    if (req.type === "text_delta") {
                        setResponse((current): ChatData => {
                            if (req.content.is_reasoning) {
                                return {
                                    ...current,
                                    response: current.response,
                                    reasoning: current.reasoning.length
                                        ? [
                                            ...current.reasoning.slice(
                                                0,
                                                Math.max(current.reasoning.length - 2, 0),
                                            ),
                                            `${current.reasoning[current.reasoning.length - 1]}${req.content.text}`,
                                        ]
                                        : [req.content.text],
                                    toolCalls: current.toolCalls,
                                };
                            } else {
                                return {
                                    ...current,
                                    reasoning: current.reasoning,
                                    response: `${current.response}${req.content.text}`,
                                    toolCalls: current.toolCalls,
                                };
                            }
                        });
                        return;
                    }
                    if (req.type === "reasoning_summary") {
                        setResponse((current) => {
                            return {
                                ...current,
                                reasoningSummary: req.content.text,
                            };
                        });
                    }
                    if (req.type === "reasoning_block") {
                        setResponse((current) => {
                            return {
                                ...current,
                                reasoning: [...current.reasoning, req.content.text],
                            };
                        });
                    }
                    if (req.type === "tool_call") {
                        consola.info(`The ${req.content.tool_name} was called!`);
                        return;
                    }
                    if (req.type === "many") {
                        for (const k of req.content) {
                            handleIndividualRequest(k);
                        }
                    }
                    if (req.type === "user_message_bounce_back") {
                        window.dispatchEvent(
                            new CustomEvent("append-user-event", {
                                detail: req.content,
                            }),
                        );
                    }
                };
                handleIndividualRequest(chatEvent);
            } catch (err: unknown) {
                consola.error("Error: ", err);
            }
        };

        ws.onerror = (error) => {
            consola.error("WebSocket error:", error);
        };

        ws.onclose = () => {
            consola.log("WebSocket disconnected");
            setConnected(false);
        };

        setInitialized(true);

        return () => {
            ws.close();
        };
    }, []);

    function sendMessage(input: string) {
        if (!socket.current || socket.current.readyState !== WebSocket.OPEN) {
            consola.warn("No socket found. Cannot continue.");
            return;
        }
        setActivelyStreaming(true);

        const data: UserMessageInput = {
            convo_id: conversation_id,
            agent_id: agent_id ?? null,
            body: input,
        };

        setResponse(getEmptyChatData());

        socket.current.send(JSON.stringify(data));
    }

    return {
        ref: container,
        response,
        connected,
        sendMessage,
        activelyStreaming,
    };
};
