import { type AppState } from "@/state/initial_state";
import React, { type ReactNode } from "react";

import { connect } from "react-redux";
import { type NotificationState } from "../state/notification_state";
import { NotificationItemComponent } from "./notification_item";
const connector = connect((state: AppState) => ({
    notifications: state.notification.notifications,
}));

interface NotificationsListProps {
    notifications: NotificationState["notifications"];
}

export const NotificationsList = connector(
    ({ notifications }: NotificationsListProps): ReactNode => {
        if (!notifications.length) {
            return null;
        }
        return (
            <div className="w-[min(450px,90vw)] fixed bottom-0 right-0 z-10">
                {notifications.map((n) => {
                    return <NotificationItemComponent item={n} key={n.id} />;
                })}
            </div>
        );
    },
);

NotificationsList.displayName = "NotificationsList";
