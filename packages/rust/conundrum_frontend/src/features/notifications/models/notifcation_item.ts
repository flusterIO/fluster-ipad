export interface NotificationItem {
    id: string;
    title: string;
    body?: string;
    /**
     * The timeout to remove the notification in milliseconds
     */
    timeout?: number;
}
