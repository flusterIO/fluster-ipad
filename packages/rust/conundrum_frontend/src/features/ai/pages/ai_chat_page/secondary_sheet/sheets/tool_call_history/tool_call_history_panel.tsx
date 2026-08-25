import { motion } from "framer-motion";
import {
    InputGroup,
    InputGroupAddon,
    InputGroupInput,
} from "@/components/shad/input-group";
import { SearchIcon } from "lucide-react";
import React, { useState, type ReactNode } from "react";
import { useSearchParams } from "react-router";
import { ToolCallHistoryInner } from "./tool_call_history_inner";
import { SheetContainerOrEmpty } from "../sheet_container_or_empty";
import { sortByCtime } from "#/database/shared_queries/sort_queries";
import { CenteredExpandedLoadingIndicator } from "#/navigation/full_screen_loading";
import { rspc } from "@/app/rspc_client";
import { type ToolExecution } from "@/codegen/bindings";
import { ToolCallItem } from "./tool_call_item";
import { ChatPanelEmptyLabel } from "../empty_label";

type ValidPredicate = `${keyof ToolExecution} = "${string}"`;

export const ToolCallHistorySheet = ({
    convo,
}: {
    convo: string;
}): ReactNode => {
    const { data: toolCalls, isLoading } = rspc.useQuery([
        "crud.tool_execution.get_by_predicate",
        {
            predicate: `convo_id = "${convo}"` satisfies ValidPredicate,
            pagination: {
                page: 1,
                per_page: 10,
            },
            sort: [sortByCtime],
        },
    ]);

    if (isLoading) {
        return <CenteredExpandedLoadingIndicator className="grow" />;
    }
    return (
        <>
            <InputGroup className="px-4 max-w-[calc(100%-2rem)] mx-auto text-sm focus-visible:ring-0! focus-visible:border-none">
                <InputGroupInput className="p-0" />
                <InputGroupAddon>
                    <SearchIcon />
                </InputGroupAddon>
            </InputGroup>
            <SheetContainerOrEmpty
                data={toolCalls}
                Content={({ data }) => {
                    return data.map((item) => {
                        return <ToolCallItem item={item} />;
                    });
                }}
                Empty={() => {
                    return (
                        <ChatPanelEmptyLabel
                            title="No Tool History"
                            body="Your tool history for this chat is empty."
                        />
                    );
                }}
            />
        </>
    );
};

ToolCallHistorySheet.displayName = "ChatSelectionSheet";
