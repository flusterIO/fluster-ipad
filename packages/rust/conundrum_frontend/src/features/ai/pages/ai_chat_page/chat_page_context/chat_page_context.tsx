"use client";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { type ReactNode, createContext, useReducer, useContext, useMemo, useState } from "react";
import { useChat, isEmptyChatResponse } from "../use_chat";

export interface ChatPageState {
    /**
     * True while loading messages from the db or other DB work.
     */
    loading: boolean;
    /**
     * True when the model is streaming.
     */
    thinking: boolean;
}

const defaultInitialValues: ChatPageState = {
    loading: false,
    thinking: false,
};

export const ChatPageContext =
    createContext<ChatPageState>(defaultInitialValues);

type ChatPageContextActions =
    | {
        type: "set-loading";
        payload: boolean;
    }
    | {
        type: "set-thinking";
        payload: boolean;
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
        default: {
            return state;
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

    /* const [sheetOpen, setSheetOpen] = useState(false); */
    /* const [hasMessages, setHasMessages] = useState(false); */
    /* useEventListener("set-ai-message-count", (e) => { */
    /*     setHasMessages(e.detail.has_messages); */
    /* }); */
    /* const { sendMessage, ref, response, activelyStreaming } = useChat(); */
    /* const showingEmptyChat = useMemo(() => { */
    /*     return isEmptyChatResponse(response) && !hasMessages; */
    /* }, [response, hasMessages]); */

    return (
        <ChatPageContext.Provider value={state}>
            <ChatPageDispatchContext.Provider value={dispatch}>
                {children}
            </ChatPageDispatchContext.Provider>
        </ChatPageContext.Provider>
    );
};
