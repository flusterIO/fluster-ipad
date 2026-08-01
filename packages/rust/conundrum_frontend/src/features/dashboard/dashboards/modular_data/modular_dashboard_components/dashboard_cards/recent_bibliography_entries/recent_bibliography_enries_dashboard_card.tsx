import React, { type ReactNode } from 'react'
import { ModularDashboardCard } from '../../modular_dashboard_card'

export const RecentBibEntriesDashboardCard = (): ReactNode => {
    return (
        <ModularDashboardCard title="Bibliography">
            Recent Bib Entries
        </ModularDashboardCard>
    )
}


RecentBibEntriesDashboardCard.displayName = "RecentBibEntriesDashboardCard"
