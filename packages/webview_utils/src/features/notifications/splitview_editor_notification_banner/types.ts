export interface EditorNotificationBanner {
    title: string;
    id: string;
    body?: string;
    /**
     * If undefined, will require user to dismiss
     * the notification manually.
     */
    timeout?: number;
}
