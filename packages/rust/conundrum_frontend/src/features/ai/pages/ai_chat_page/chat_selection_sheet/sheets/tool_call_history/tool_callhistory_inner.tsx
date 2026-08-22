import { sortByCtime } from "#/database/shared_queries/sort_queries";
import { CenteredExpandedLoadingIndicator } from "#/navigation/full_screen_loading";
import { rspc } from "@/app/rspc_client";
import React, { type ReactNode } from "react";
import { ToolCallItem } from "./tool_call_item";

interface ToolCallHistoryInnerProps {
    convo: string;
}

export const ToolCallHistoryInner = ({
    confo,
}: ToolCallHistoryInnerProps): ReactNode => {
    const { data: conversations, isLoading } = rspc.useQuery([
        "crud.tool_ex",
        {
            predicate: undefined,
            pagination: {
                page,
                per_page: 10,
            },
            sort: [sortByCtime],
        },
    ]);

    if (isLoading) {
        return (
            <CenteredExpandedLoadingIndicator className="grow" />
        )

    }
    return (
        conversations?.length ? (
            conversations.map((c) => {
                return <ToolCallItem item={c} key={c.id} />;
            })
        ) : (
            <div className="grow w-full h-full flex flex-col justify-center items-center">
                <h6 className="text-center text-lg font-semibold text-foreground">
                    No History
                </h6>
                <div className="text-center text-foreground/80">
                    You don't have any chat history to display. Simply start a
                    conversation and AI will take care of the rest
                </div>
            </div>
        )
};

ToolCallHistoryInner.displayName = "ToolCallHistoryInner";
