import { type ChatConversationResult } from "#/database/db_utility_types/chat";
import { DateTimeComponent } from "#/datetime/components/date_time";
import React, { type ReactNode } from "react";

interface ChatSelectionItemProps {
    item: ChatConversationResult[number];
}

export const ChatSelectionItem = ({
    item,
}: ChatSelectionItemProps): ReactNode => {
    return (
        <div className="w-full h-fit flex flex-col justify-start items-start p-2 bg-fd-card text-fd-card-foreground rounded">
            <h5 className="font-bold">{item.label}</h5>
            <div className="font-bold">
                <DateTimeComponent dateTime={item.ctime} format="full-with-time" />
            </div>
        </div>
    );
};

ChatSelectionItem.displayName = "ChatSelectionItem";
