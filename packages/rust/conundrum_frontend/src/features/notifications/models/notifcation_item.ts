import { type EcosystemLogInput } from "#/database/db_utility_types/log_types";
import { type EcosystemLogSeverity } from "@/codegen/bindings";
import { v4 } from "uuid";

export interface NotificationItem {
    id: string;
    title: string;
    body?: string;
    severity?: EcosystemLogSeverity;
    /**
     * The timeout to remove the notification in milliseconds
     */
    timeout?: number;
}

export const ecosystemLogToNotification = (
    log: EcosystemLogInput,
): NotificationItem => {
    return {
        id: v4(),
        title: log.title,
        body: log.message ?? undefined,
        severity: log.severity,
        timeout: 5000,
    };
};
