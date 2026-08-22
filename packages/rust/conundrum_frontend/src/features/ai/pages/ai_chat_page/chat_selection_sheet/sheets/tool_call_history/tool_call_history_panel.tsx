import { sortByCtime } from "#/database/shared_queries/sort_queries";
import { CenteredExpandedLoadingIndicator } from "#/navigation/full_screen_loading";
import { rspc } from "@/app/rspc_client";
import { motion } from "framer-motion";
import {
    InputGroup,
    InputGroupAddon,
    InputGroupInput,
} from "@/components/shad/input-group";
import { SheetHeader, SheetTitle } from "@/components/shad/sheet";
import { SearchIcon } from "lucide-react";
import React, { useState, type ReactNode } from "react";
import { useSearchParams } from "react-router";
import { ToolCallItem } from "./tool_call_item";
import { ToolCallHistoryInner } from "./tool_callhistory_inner";

export const ToolCallHistorySheet = (): ReactNode => {
    const [sp, setSp] = useSearchParams();
    const convo = sp.get("convo");
    const [page, setPage] = useState(1);
    return (
        <>
            <SheetHeader className="w-full">
                <SheetTitle className="w-full">Tool Call History</SheetTitle>
            </SheetHeader>
            <div className="px-4 grow w-full">
                <InputGroup className="px-4 text-sm focus-visible:ring-0! focus-visible:border-none">
                    <InputGroupInput className="p-0" />
                    <InputGroupAddon>
                        <SearchIcon />
                    </InputGroupAddon>
                </InputGroup>
                {convo ? (
                    <ToolCallHistoryInner convo={convo} />
                ) : (
                    <div className="w-full grow flex flex-col justify-center items-center p-3">
                        <div>
                            <h3>No Conversation</h3>
                            <p className="text-foreground/80 font-sm">
                                You must be in an active conversation to use this panel
                            </p>
                        </div>
                    </div>
                )}
            </div>
        </>
    );
};

ToolCallHistorySheet.displayName = "ChatSelectionSheet";
