import { sortByCtime } from "#/database/shared_queries/sort_queries";
import { CenteredExpandedLoadingIndicator } from "#/navigation/full_screen_loading";
import { rspc } from "@/app/rspc_client";
import {
    InputGroup,
    InputGroupAddon,
    InputGroupInput,
} from "@/components/shad/input-group";
import {
    Sheet,
    SheetContent,
    SheetHeader,
    SheetTitle,
} from "@/components/shad/sheet";
import { SearchIcon } from "lucide-react";
import React, { useState, type ReactNode } from "react";
import { useSearchParams } from "react-router";
import { ChatSelectionItem } from "./chat_selection_item";

export const ChatSelectionSheet = ({
    open,
    close,
}: {
    open: boolean;
    close: () => void;
}): ReactNode => {
    const [sp, setSp] = useSearchParams();
    const convo = sp.get("convo");
    const [page, setPage] = useState(1);
    const { data: conversations, isLoading } = rspc.useQuery([
        "crud.chat_conversation.get_by_predicate",
        {
            predicate: convo ? `conversation_id="${convo}"` : undefined,
            pagination: {
                page,
                per_page: 10,
            },
            sort: [sortByCtime],
        },
    ]);
    return (
        <Sheet
            open={open}
            onOpenChange={(val) => {
                if (!val) {
                    close();
                }
            }}
        >
            <SheetContent
                side="right"
                className="flex flex-col justify-start items-center"
            >
                <SheetHeader className="w-full">
                    <SheetTitle className="w-full">Chat History</SheetTitle>
                </SheetHeader>
                <div className="px-4 grow w-full">
                    <InputGroup className="px-4 text-sm focus-visible:ring-0! focus-visible:border-none">
                        <InputGroupInput className="p-0" />
                        <InputGroupAddon>
                            <SearchIcon />
                        </InputGroupAddon>
                    </InputGroup>
                    {isLoading ? (
                        <CenteredExpandedLoadingIndicator className="grow" />
                    ) : conversations?.length ? (
                        conversations.map((c) => {
                            return <ChatSelectionItem item={c} key={c.id} />;
                        })
                    ) : (
                        <div className="grow w-full h-full flex flex-col justify-center items-center">
                            <h6 className="text-center text-lg font-semibold">No History</h6>
                            <div className="text-center">
                                You don't have any chat history to display. Simply start a
                                conversation and AI will take care of the rest
                            </div>
                        </div>
                    )}
                </div>
            </SheetContent>
        </Sheet>
    );
};

ChatSelectionSheet.displayName = "ChatSelectionSheet";
