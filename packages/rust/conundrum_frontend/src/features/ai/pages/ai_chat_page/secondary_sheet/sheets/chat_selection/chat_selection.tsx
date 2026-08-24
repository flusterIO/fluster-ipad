import { sortByCtime } from "#/database/shared_queries/sort_queries";
import { CenteredExpandedLoadingIndicator } from "#/navigation/full_screen_loading";
import { rspc } from "@/app/rspc_client";
import { motion } from "framer-motion";
import {
    InputGroup,
    InputGroupAddon,
    InputGroupInput,
} from "@/components/shad/input-group";
import { SearchIcon } from "lucide-react";
import React, { useState, type ReactNode } from "react";
import { ChatList } from "./chat_list";
import { useChatPageContext } from "../../../chat_page_context/chat_page_context";

export const ChatSelectionSheet = (): ReactNode => {
    const { page } = useChatPageContext();
    const [inputValue, setInputValue] = useState("")
    const { data: conversations, isLoading } = rspc.useQuery([
        "crud.chat_conversation.get_by_predicate",
        {
            predicate: !inputValue.trim().length ? undefined : `label CONTAINS "${inputValue}"`,
            pagination: {
                page,
                per_page: 10,
            },
            sort: [sortByCtime],
        },
    ]);
    return (
        <>
            <motion.div
                className="px-4 grow w-full"
                initial={{
                    opacity: 0,
                }}
                animate={{
                    opacity: 1,
                }}
                exit={{
                    opacity: 0,
                    scale: 0,
                }}
            >
                <InputGroup className="px-4 text-sm focus-visible:ring-0! focus-visible:border-none">
                    <InputGroupInput value={inputValue} onChange={(e) => {
                        setInputValue(e.target.value)
                    }} className="p-0" />
                    <InputGroupAddon>
                        <SearchIcon />
                    </InputGroupAddon>
                </InputGroup>
                {isLoading ? (
                    <CenteredExpandedLoadingIndicator className="grow" />
                ) : conversations?.length ? (
                    <ChatList items={conversations} />
                ) : (
                    <div className="grow w-full h-full flex flex-col justify-center items-center">
                        <h6 className="text-center text-lg font-semibold text-foreground">
                            No History
                        </h6>
                        <div className="text-center text-foreground/80">
                            You don't have any chat history to display. Simply start a
                            conversation and we'll take care of the rest
                        </div>
                    </div>
                )}
            </motion.div>
        </>
    );
};

ChatSelectionSheet.displayName = "ChatSelectionSheet";
