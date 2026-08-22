import { type ChatConversationResult } from "#/database/db_utility_types/chat";
import React, { type ReactNode } from "react";
import { ChatConversationItem } from "./chat_convo_item";
import { buttonVariants } from "@/components/shad/button";
import { Link } from "react-router";
import { cn } from "@/utils/shad_utils";
import { v4 } from "uuid";
import { AppPaths } from "#/navigation/app_paths";
import { useChatPageDispatch } from "../../../chat_page_context/chat_page_context";

interface ChatListProps {
    items: ChatConversationResult;
}

export const ChatList = ({ items }: ChatListProps): ReactNode => {
    const sp = new URLSearchParams();
    const dispatch = useChatPageDispatch();
    sp.set("convo", v4());
    return (
        <div className="w-full h-fit flex flex-col justify-start items-start gap-y-2 py-4">
            {items.map((item) => {
                return <ChatConversationItem item={item} key={item.id} />;
            })}
            <Link
                className={cn(
                    buttonVariants({ variant: "secondary" }),
                    "w-full mx-auto hover:bg-secondary/80",
                )}
                to={`${AppPaths.aiChat}?${sp.toString()}`}
                onClick={() => {
                    dispatch({
                        type: "set-sheet-open",
                        payload: false,
                    });
                    dispatch({
                        type: "set-response",
                        payload: null,
                    });
                }}
            >
                New Chat
            </Link>
        </div>
    );
};

ChatList.displayName = "ChatList";
