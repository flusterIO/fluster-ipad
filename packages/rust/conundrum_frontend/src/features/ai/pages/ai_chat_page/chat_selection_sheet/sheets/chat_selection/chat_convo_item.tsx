import { type ChatConversationResult } from "#/database/db_utility_types/chat";
import { DateTimeComponent } from "#/datetime/components/date_time";
import React, { type ReactNode } from "react";
import { useNavigate, useSearchParams } from "react-router";
import { cn } from "@/utils/shad_utils";
import { AppPaths } from "#/navigation/app_paths";
import { useChatPageDispatch } from "../../../chat_page_context/chat_page_context";

interface ChatConversationItemProps {
    item: ChatConversationResult[number];
}

export const ChatConversationItem = ({
    item,
}: ChatConversationItemProps): ReactNode => {
    const [sp] = useSearchParams();
    const chatId = sp.get("convo");
    const newSp = new URLSearchParams();
    newSp.set("convo", item.id);
    const navigate = useNavigate();
    const dispatch = useChatPageDispatch();
    return (
        <div
            className={cn(
                "w-full h-fit rounded bg-fd-card p-3 transition-colors duration-300 border border-border",
                chatId === item.id &&
                "border-primary!",
            )}
            onClick={() => {
                if (chatId !== item.id) {
                    // eslint-disable-next-line @typescript-eslint/no-floating-promises
                    navigate(`${AppPaths.aiChat}?${newSp.toString()}`);
                    dispatch({
                        type: "set-response",
                        payload: null,
                    });
                }
            }}
        >
            <div className="text-foreground font-bold">
                {item.label.length ? item.label : "Unlabeled Chat"}
            </div>
            <DateTimeComponent
                format="full-with-time"
                className="text-sm text-foreground/70"
                dateTime={item.ctime}
            />
        </div>
    );
};

ChatConversationItem.displayName = "ChatConversationItem";
