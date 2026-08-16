import React, { type ReactNode } from "react";
import { SecondaryPanelContent } from "../../secondary_panel_content";
import { rspc } from "@/app/rspc_client";
import { SidePanelLogItem } from "./log_item";
import { EmptyPanel } from "../../empty_panel";

export const LogPanel = (): ReactNode => {
    const { data } = rspc.useQuery([
        "log.get_many",
        {
            predicate: null,
            pagination: {
                page: 1,
                per_page: 10,
            },
            sort: [
                {
                    column: "ctime",
                    order: "desc-null-first",
                },
            ],
        },
    ]);
    return (
        <SecondaryPanelContent
            title="Logs"
            desc="Your recent logs displayed in the order that they were created."
            centerChildren
        >
            {data?.length ? (
                data.map((d) => {
                    return <SidePanelLogItem item={d} key={d.id} />;
                })
            ) : (
                <EmptyPanel
                    title="No logs found"
                    desc="These logs will be automatically generated as you use the Conundrum ecosystem"
                />
            )}
        </SecondaryPanelContent>
    );
};

LogPanel.displayName = "LogPanel";
