import { type ChatMessageResult } from "#/database/db_utility_types/chat";
import { getServerPort } from "@/app/rspc_client";
import consola from "consola";
import { useEffect, useRef, useState } from "react";
import { useSearchParams } from "react-router";
import { type ChatEvent } from "@conundrum/ts/codegen-typeshare-server";

export interface ChatData {
    reasoning: string[];
    response: string;
    reasoningSummary?: string;
    toolCalls: { tool_name: string; tool_input_params?: string }[];
    tokens: {
        total?: number;
        incoming?: number;
        outgoing?: number;
    };
}

export const useChat = () => {
    const container = useRef<HTMLDivElement>(null);
    const [initialized, setInitialized] = useState(false);
    const [sp, setSp] = useSearchParams();
    const [messages, setMessages] = useState<ChatMessageResult>([]);
    const [activelyStreaming, setActivelyStreaming] = useState(false);
    const [response, setResponse] = useState<ChatData>({
        reasoning: [],
        response: "",
        toolCalls: [],
        tokens: {
            total: undefined,
            incoming: undefined,
            outgoing: undefined,
        },
    });
    const [connected, setConnected] = useState(false);

    const chatId = sp.get("convo");
    const page = sp.get("page") ?? "1";

    const socket = useRef<WebSocket | null>(null);

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
                console.log("chatEvent: ", chatEvent);

                const handleIndividualRequest = (req: ChatEvent) => {
                    if (req.type !== "done") {
                        setActivelyStreaming(true);
                    } else {
                        setActivelyStreaming(false);
                        return;
                    }
                    if (req.type === "text_delta") {
                        setResponse((current) => {
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

        setResponse({
            response: "",
            reasoning: [],
            toolCalls: [],
            tokens: {},
        });

        socket.current.send(input);
    }

    return {
        ref: container,
        response,
        connected,
        sendMessage,
        messages,
        activelyStreaming,
    };
};
