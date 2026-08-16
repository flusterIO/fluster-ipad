import React, { type ReactNode } from "react";
import { type NotificationItem } from "../models/notifcation_item";

interface NotificationItemComponentProps {
    item: NotificationItem;
}

export const NotificationItemComponent = ({
    item,
}: NotificationItemComponentProps): ReactNode => {
    return (
        <div className="border rounded w-full h-fit flex flex-col justify-start items-start px-3 py-2">
            <div className="font-bold">{item.title}</div>
            {item.body ? (
                <div className="text-foreground/60">{item.title}</div>
            ) : null}
        </div>
    );
};

NotificationItemComponent.displayName = "NotificationItemComponent";
