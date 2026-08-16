import { type EcosystemLogItem } from "#/database/db_utility_types/log_types";
import React, { type ReactNode } from "react";

interface SidePanelLogItemProps {
    item: EcosystemLogItem;
}

export const SidePanelLogItem = ({
    item,
}: SidePanelLogItemProps): ReactNode => {
    return (
        <div className="w-full h-fit rounded border bg-fd-card/50">
            <div className="font-semibold">{item.title}</div>
            {item.message ? (
                <div className="text-foreground/60">{item.message}</div>
            ) : null}
        </div>
    );
};

SidePanelLogItem.displayName = "SidePanelLogItem";
