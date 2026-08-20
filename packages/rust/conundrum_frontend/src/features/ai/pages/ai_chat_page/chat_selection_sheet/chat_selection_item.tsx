import { type ChatConversationResult } from "#/database/db_utility_types/chat";
import { DateTimeComponent } from "#/datetime/components/date_time";
import React, { type ReactNode } from "react";
import { motion } from "framer-motion";
import { cn } from "@/utils/shad_utils";

interface ChatSelectionItemProps {
    item: ChatConversationResult[number];
    active: boolean;
}

export const ChatSelectionItem = ({
    item,
    active,
}: ChatSelectionItemProps): ReactNode => {
    return (
        <motion.div
            className={cn(
                "w-full h-fit flex flex-col justify-start items-start p-2 bg-fd-card text-fd-card-foreground rounded border",
                active ? "border-primary/50!" : "border-border",
            )}
        >
            <h5 className="font-bold">{item.label}</h5>
            <div className="font-bold">
                <DateTimeComponent dateTime={item.ctime} format="full-with-time" />
            </div>
        </motion.div>
    );
};

ChatSelectionItem.displayName = "ChatSelectionItem";
