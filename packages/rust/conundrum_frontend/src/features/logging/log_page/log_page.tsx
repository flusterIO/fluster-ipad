import { sortByCtime } from "#/database/shared_queries/sort_queries";
import { CenteredExpandedLoadingIndicator } from "#/navigation/full_screen_loading";
import { rspc } from "@/app/rspc_client";
import { PageContainer } from "@/components/general/page_container";
import React, { type ReactNode } from "react";
import { LogItemComponent } from "./log_item/log_item";

export const LogPage = (): ReactNode => {
    const { data: logs = [], isLoading } = rspc.useQuery([
        "log.get_many",
        {
            pagination: {
                page: 1,
                per_page: 50,
            },
            predicate: null,
            sort: [sortByCtime],
        },
    ]);
    return (
        <PageContainer
            itemClasses="flex flex-col justify-start items-center gap-y-4"
            title="Ecosystem Logs"
            center={!logs.length || isLoading}
        >
            {isLoading ? (
                <CenteredExpandedLoadingIndicator />
            ) : (
                logs.map((l) => {
                    return <LogItemComponent item={l} key={l.id} />;
                })
            )}
        </PageContainer>
    );
};

LogPage.displayName = "LogPage";
