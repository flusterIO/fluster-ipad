import { type Procedures } from "@/codegen/bindings";

export type ChatMessageResult =
    Procedures["crud"]["user_message"]["get_by_predicate"]["output"];

export type ChatHistoryResponse =
    Procedures["agent"]["load_chat_history"]["output"];

export type ChatMessageResultItem = ChatMessageResult[number];

export type ChatConversationResult =
    Procedures["crud"]["chat_conversation"]["get_by_predicate"]["output"];

export type ChatClientData = Procedures["agent"]["save_chat_data"]["input"];

export type ToolExecutionPartial = ChatClientData["tool_calls"][number];
