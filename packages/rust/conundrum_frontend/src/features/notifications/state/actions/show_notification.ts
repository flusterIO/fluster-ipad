import { type NotificationItem } from "#/notifications/models/notifcation_item";
import store from "@/state/store";
import { appendNotification } from "../notification_state_slice";
import { v4 } from "uuid";

export const showNotification = (notif: Omit<NotificationItem, "id">) => {
    store.dispatch(
        appendNotification({
            id: v4(),
            ...notif,
        }),
    );
};

export const showErrorNotification = ({
    title,
    timeout = 5000,
    body,
}: {
    title: string;
    body?: string;
    timeout: number;
}) => {
    showNotification({
        title,
        body,
        timeout,
    });
};
