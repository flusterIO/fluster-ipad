import React, { useEffect, type ReactNode } from "react";
import { type NotificationItem } from "../models/notifcation_item";
import { useDispatch } from "react-redux";
import { removeNotificationById } from "../state/notification_state_slice";
import { XIcon } from "lucide-react";
import { motion } from "framer-motion";

interface NotificationItemComponentProps {
    item: NotificationItem;
}

export const NotificationItemComponent = ({
    item,
}: NotificationItemComponentProps): ReactNode => {
    const dispatch = useDispatch();
    useEffect(() => {
        if (item.timeout) {
            setTimeout(() => {
                dispatch(removeNotificationById(item.id));
            }, item.timeout);
        }
    }, [item]);
    return (
        <motion.div
            className="border rounded w-full h-fit flex flex-col justify-start items-start px-3 py-2 bg-fd-card text-fd-card-foreground"
            initial={{
                x: "100%",
                opacity: 0,
            }}
            animate={{
                x: 0,
                opacity: 1,
            }}
            exit={{
                x: "100%",
                opacity: 0,
            }}
        >
            <div className="font-bold text-foreground w-full relative pr-3">
                {item.title}
                <XIcon
                    className="absolute right-0 top-0 w-3 h-3"
                    onClick={() => {
                        dispatch(removeNotificationById(item.id));
                    }}
                />
            </div>
            {item.body ? <div className="text-sm opacity-80">{item.body}</div> : null}
        </motion.div>
    );
};

NotificationItemComponent.displayName = "NotificationItemComponent";
