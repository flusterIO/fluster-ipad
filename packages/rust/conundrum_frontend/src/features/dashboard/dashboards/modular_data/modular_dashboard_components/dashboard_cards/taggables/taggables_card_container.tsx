import React, { useState, type ReactNode } from 'react'
import { ModularDashboardCard } from '../../modular_dashboard_card'

export const TaggablesDashboardCard = (): ReactNode => {
    const [variant] = useState<"tag" | "topic" | "subject">("tag");
    return (
        <ModularDashboardCard title={{
            tag: "Tags",
            topic: "Topics",
            subject: "Subjects"
        }[variant]} center>
            {variant}
        </ModularDashboardCard>
    )
}


TaggablesDashboardCard.displayName = "TaggablesDashboardCard"
