import { type Procedures } from "@/codegen/bindings";

export type ChatMessageResult =
    Procedures["crud"]["chat_message"]["get_by_predicate"]["output"];

export type ChatMessageResultItem = ChatMessageResult[number];

export type ChatConversationResult =
    Procedures["crud"]["chat_conversation"]["get_by_predicate"]["output"];
