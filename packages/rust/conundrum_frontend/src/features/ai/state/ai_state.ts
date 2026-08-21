export interface AIState {
    chatAgentID: string | null;
    /**
     * The number of milliseconds after midnight that the time daily chat is expect to expire.
     * Obviously don't make the user insert this as milliseconds.
     */
    dailyChatTimeExpires: null | number;
    dailyChat: {
        expires_at: string;
        chat_id: string;
    } | null;
}
