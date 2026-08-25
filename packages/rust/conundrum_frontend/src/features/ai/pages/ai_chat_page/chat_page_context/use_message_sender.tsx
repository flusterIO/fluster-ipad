import consola from "consola";
import { useChatPageContext, useChatPageDispatch } from "./chat_page_context";
import { type UserMessageInput } from "@conundrum/ts/codegen-typeshare";
import { getEmptyChatData } from "../use_chat";

export const useMessageSender = () => {
    const { socket, convo, agent, ws_connected } = useChatPageContext();
    const dispatch = useChatPageDispatch();
    function sendMessage(input: string) {
        if (!ws_connected || !socket?.OPEN) {
            consola.warn("No socket found. Cannot continue.");
            return;
        }

        dispatch({
            type: "set-thinking",
            payload: true,
        });

        const data: UserMessageInput = {
            convo_id: convo ?? undefined,
            agent_id: agent ?? undefined,
            body: input,
        };

        dispatch({
            type: "append-message",
            payload: {
                type: "user-partial",
                data: {
                    ...data,
                    ctime: new Date(),
                },
            },
        });

        dispatch({
            type: "set-response",
            payload: getEmptyChatData(),
        });

        socket.send(JSON.stringify(data));
    }

    return sendMessage;
};
