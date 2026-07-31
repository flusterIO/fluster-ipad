import React, { type ReactNode } from 'react'



interface RecentNoteDashboardListItemProps {
    item: {
        title?: string
        ctime?: Date
    }
}

export const RecentNoteDashboardListItem = ({ item }: RecentNoteDashboardListItemProps): ReactNode => {
    return (
        <div className="w-full h-fit">
            <div>{item.title}</div>
        </div>
    )
}


RecentNoteDashboardListItem.displayName = "RecentNoteDashboardListItem"
