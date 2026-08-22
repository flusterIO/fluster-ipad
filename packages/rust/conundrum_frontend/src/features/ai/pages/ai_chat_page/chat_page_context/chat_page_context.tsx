"use client";
import { useSelector } from "react-redux";
import { v4 } from "uuid";
import {
    type ReactNode,
    createContext,
    useReducer,
    useContext,
    useState,
    useEffect,
    useCallback,
    useRef,
} from "react";
import {
    isEmptyChatResponse,
    type ChatData,
    getEmptyChatData,
} from "../use_chat";
import { type ChatEvent } from "@conundrum/ts/codegen-typeshare";
import { useLogger } from "#/logging/state/hooks/use_logger";
import { getServerPort } from "@/app/rspc_client";
import { type AppState } from "@/state/initial_state";
import consola from "consola";
import { useSearchParams } from "react-router";
import { type FormattedChatHistoryItem } from "#/ai/state/hooks/use_formatted_chat_history";

export interface ChatPageState {
    /**
     * True while loading messages from the db or other DB work.
     */
    loading: boolean;
    /**
     * True when the model is streaming.
     */
    thinking: boolean;
    sheetOpen: boolean;
    hasMessages: boolean;
    response: ChatData | null;
    messages: FormattedChatHistoryItem[];
    initialized: boolean;
    ws_connected: boolean;
    convo: string | null;
    agent: string | null;
    socket: WebSocket | null;
    page: number;
}

const defaultInitialValues: ChatPageState = {
    loading: false,
    thinking: false,
    sheetOpen: false,
    response: null,
    hasMessages: false,
    messages: [],
    ws_connected: false,
    initialized: false,
    convo: null,
    agent: null,
    socket: null,
    page: 1,
};

export const ChatPageContext =
    createContext<ChatPageState>(defaultInitialValues);

type ChatPageContextActions =
    | {
        type: "set-loading";
        payload: boolean;
    }
    | {
        type: "set-sheet-open";
        payload: boolean;
    }
    | {
        type: "set-socket";
        payload: WebSocket | null;
    }
    | {
        type: "set-agent";
        payload: string | null;
    }
    | {
        type: "set-initialized";
        payload: boolean;
    }
    | {
        type: "set-convo";
        payload: string | null;
    }
    | {
        type: "set-ws-connected";
        payload: boolean;
    }
    | {
        type: "set-response";
        payload: ChatData | null;
    }
    | {
        type: "set-thinking";
        payload: boolean;
    }
    | {
        type: "set-has-messages";
        payload: boolean;
    }
    | {
        type: "set-messages";
        payload: FormattedChatHistoryItem[];
    }
    | {
        type: "append-message";
        payload: FormattedChatHistoryItem;
    }
    | {
        type: "set-page";
        payload: number;
    };

export const ChatPageDispatchContext = createContext<
    React.Dispatch<ChatPageContextActions>
>(null!);

export const useChatPageContext = () => useContext(ChatPageContext);
export const useChatPageDispatch = () => useContext(ChatPageDispatchContext);

export const ChatPageContextReducer = (
    state: ChatPageState,
    action: ChatPageContextActions,
): ChatPageState => {
    switch (action.type) {
        case "set-loading": {
            return {
                ...state,
                loading: state.loading,
            };
        }
        case "set-thinking": {
            return {
                ...state,
                thinking: state.thinking,
            };
        }
        case "set-has-messages": {
            return {
                ...state,
                hasMessages: action.payload,
            };
        }
        case "set-messages": {
            return {
                ...state,
                messages: action.payload,
                hasMessages: Boolean(action.payload.length)
            };
        }
        case "append-message": {
            return {
                ...state,
                messages: [...state.messages, action.payload],
            };
        }
        case "set-sheet-open": {
            return {
                ...state,
                sheetOpen: action.payload,
            };
        }
        case "set-socket": {
            return {
                ...state,
                socket: action.payload,
            };
        }
        case "set-agent": {
            return {
                ...state,
                agent: action.payload,
            };
        }
        case "set-initialized": {
            return {
                ...state,
                initialized: action.payload,
            };
        }
        case "set-convo": {
            return {
                ...state,
                convo: action.payload,
            };
        }
        case "set-ws-connected": {
            return {
                ...state,
                ws_connected: action.payload,
            };
        }
        case "set-response": {
            return {
                ...state,
                response: action.payload,
            };
        }
        case "set-page": {
            return {
                ...state,
                page: action.payload
            }
        }
    }
};

ChatPageContextReducer.displayName = "ChatPageContextReducer";

interface ChatPageProviderProps {
    children: ReactNode;
    initialValues?: Partial<ChatPageState>;
}

export const ChatPageProvider = ({
    children,
    initialValues,
}: ChatPageProviderProps) => {
    const [state, dispatch] = useReducer(
        ChatPageContextReducer,
        initialValues
            ? { ...initialValues, ...defaultInitialValues }
            : defaultInitialValues,
    );
    const dailyChat = useSelector((state: AppState) => {
        return state.ai.dailyChat;
    });
    const [sp, setSp] = useSearchParams();
    const [activelyStreaming, setActivelyStreaming] = useState(false);
    const [response, setResponse] = useState<ChatData>(getEmptyChatData());
    const logger = useLogger();

    const page = sp.get("page") ?? "1";
    const agent_id = sp.get("agent");
    const conversation_id = sp.get("convo");

    useEffect(() => {
        dispatch({
            type: "set-convo",
            payload: conversation_id ?? null,
        });
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
        dispatch({
            type: "set-agent",
            payload: agent_id ?? null,
        });
    }, [agent_id]);

    useEffect(() => {
        if (state.initialized) {
            return;
        }
        const ws = new WebSocket(`ws://127.0.0.1:${getServerPort()}/api/ws`);

        socket.current = ws;

        ws.onopen = () => {
            consola.log("WebSocket connected");
            dispatch({
                type: "set-ws-connected",
                payload: true,
            });
        };

        ws.onmessage = (event: MessageEvent<string>) => {
            try {
                const chatEvent = JSON.parse(event.data) as ChatEvent;

                const handleIndividualRequest = (req: ChatEvent) => {
                    if (req.type !== "done") {
                        setActivelyStreaming(true);
                    } else {
                        consola.log(`Tokens expended: `, req.content);
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
            dispatch({
                type: "set-ws-connected",
                payload: false,
            });
        };
        dispatch({
            type: "set-initialized",
            payload: true,
        });

        dispatch({
            type: "set-socket",
            payload: ws,
        });

        return () => {
            ws.close();
            dispatch({
                type: "set-ws-connected",
                payload: false,
            });
        };
    }, []);

    useEffect(() => {
        dispatch({
            type: "set-page",
            payload: page ? parseInt(page) : 1
        });
    }, [page]);

    useEffect(() => {
        const isEmpty = isEmptyChatResponse(response);
        dispatch({
            type: "set-response",
            payload: isEmpty ? null : response,
        });
    }, [response, state.hasMessages]);

    return (
        <ChatPageContext.Provider value={state}>
            <ChatPageDispatchContext.Provider value={dispatch}>
                {children}
            </ChatPageDispatchContext.Provider>
        </ChatPageContext.Provider>
    );
};
