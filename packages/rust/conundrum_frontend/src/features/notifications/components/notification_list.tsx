import { type AppState } from "@/state/initial_state";
import React, { type ReactNode } from "react";

import { connect } from "react-redux";
import { type NotificationState } from "../state/notification_state";
import { NotificationItemComponent } from "./notification_item";
import { AnimatePresence } from "framer-motion";
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
            <div className="w-[min(450px,90vw)] fixed bottom-0 right-0 p-3 z-10 flex flex-col justify-end items-center gap-y-3">
                <AnimatePresence>
                    {notifications.map((n) => {
                        return <NotificationItemComponent item={n} key={n.id} />;
                    })}
                </AnimatePresence>
            </div>
        );
    },
);

NotificationsList.displayName = "NotificationsList";
