import React, { type ReactNode } from "react";
import { ModularDashboardCard } from "../../modular_dashboard_card";
import { EmptyCardDataText } from "../empty_card_text";

export const TopicsDashboardCard = (): ReactNode => {
    return (
        <ModularDashboardCard title="Topics">
            <EmptyCardDataText>No tags found</EmptyCardDataText>
        </ModularDashboardCard>
    );
};

TopicsDashboardCard.displayName = "TopicsDashboardCard";
