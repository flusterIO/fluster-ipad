export interface AIState {
    chatAgentID: string | null;
    dailyChat: {
        expires_at: string;
        chat_id: string;
    } | null
}
