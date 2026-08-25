import { type UserMessageInput } from "@conundrum/ts/codegen-typeshare";
import { type ChatData } from "../use_chat";

export const responseToUserMessage = (
    res: ChatData,
    convo_id: string,
    agent_id?: string,
): UserMessageInput => {
    return {
        body: res.response,
        agent_id,
        convo_id,
    };
};
