import { type ChatConversationResult } from "#/database/db_utility_types/chat";
import { DateTimeComponent } from "#/datetime/components/date_time";
import React, { type ReactNode } from "react";
import { useNavigate, useSearchParams } from "react-router";
import { cn } from "@/utils/shad_utils";
import { AppPaths } from "#/navigation/app_paths";
import { useChatPageDispatch } from "../../../chat_page_context/chat_page_context";
import {
    ContextMenu,
    ContextMenuContent,
    ContextMenuPortal,
    ContextMenuTrigger,
    ContextMenuItem,
} from "@/components/shad/context-menu";
import { rspc } from "@/app/rspc_client";
import consola from "consola";

interface ChatConversationItemProps {
    item: ChatConversationResult[number];
    refetch: () => void;
}

export const ChatConversationItem = ({
    item,
    refetch,
}: ChatConversationItemProps): ReactNode => {
    const [sp] = useSearchParams();
    const { mutateAsync: deleteConversation } = rspc.useMutation(
        "crud.chat_conversation.delete_conversation",
    );
    const chatId = sp.get("convo");
    const newSp = new URLSearchParams();
    newSp.set("convo", item.id);
    const navigate = useNavigate();
    const dispatch = useChatPageDispatch();
    return (
        <ContextMenu>
            <ContextMenuTrigger
                render={(props) => {
                    return (
                        <div
                            {...props}
                            className={cn(
                                "w-full h-fit rounded bg-fd-card p-3 transition-colors duration-300 border border-border",
                                chatId === item.id && "border-primary!",
                            )}
                            onClick={() => {
                                if (chatId !== item.id) {
                                    dispatch({
                                        type: "set-response",
                                        payload: null,
                                    });
                                    dispatch({
                                        type: "set-sheet-open",
                                        payload: false,
                                    });
                                    // eslint-disable-next-line @typescript-eslint/no-floating-promises
                                    navigate(`${AppPaths.aiChat}?${newSp.toString()}`);
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
                }}
            />
            <ContextMenuPortal>
                <ContextMenuContent>
                    <ContextMenuItem
                        onClick={() => {
                            deleteConversation(item.id).catch((err: unknown) => {
                                consola.error("Error: ", err);
                            });
                            refetch();
                        }}
                    >
                        Delete
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenuPortal>
        </ContextMenu>
    );
};

ChatConversationItem.displayName = "ChatConversationItem";
