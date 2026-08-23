export interface AIState {
    chatAgentID: string | null;
    /**
     * The number of milliseconds after midnight that the time daily chat is expect to expire.
     * Obviously don't make the user insert this as milliseconds.
     */
    dailyChatTimeExpires: null | number;
    mostRecentChat: string | null;
    dailyChat: {
        /**
         * True after the user was directed to this daily chat so that the user isn't directed here more than once. THis should give the behavior of being directed to the 'daily chat' once per day, and then following the 'most recent' chat the rest of the day.
         */
        was_directed: boolean;
        expires_at: string;
        chat_id: string;
    } | null;
}
