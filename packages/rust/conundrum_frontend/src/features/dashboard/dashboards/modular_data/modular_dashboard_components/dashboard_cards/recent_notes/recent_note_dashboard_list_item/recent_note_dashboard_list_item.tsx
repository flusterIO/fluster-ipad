import { CdrmContent } from "#/cdrm/cdrm_content";
import { type CDRMModelItem } from "#/database/db_utility_types/cdrm";
import React, { type ReactNode } from "react";

interface RecentNoteDashboardListItemProps {
    item: CDRMModelItem;
}

export const RecentNoteDashboardListItem = ({
    item,
}: RecentNoteDashboardListItemProps): ReactNode => {
    return (
        <div className="w-full h-fit bg-fd-card text-foreground">
            <CdrmContent
                className="text-lg"
                inline
                content={item.title ?? "-- Unknown Title --"}
            />
        </div>
    );
};

RecentNoteDashboardListItem.displayName = "RecentNoteDashboardListItem";
