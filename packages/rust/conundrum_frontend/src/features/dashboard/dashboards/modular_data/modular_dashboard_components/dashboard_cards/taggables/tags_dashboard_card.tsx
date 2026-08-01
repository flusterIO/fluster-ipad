import React, { type ReactNode } from "react";
import { ModularDashboardCard } from "../../modular_dashboard_card";
import { EmptyCardDataText } from "../empty_card_text";

export const TagsDashboardCard = (): ReactNode => {
    return (
        <ModularDashboardCard title="Tags">
            <EmptyCardDataText>No tags found</EmptyCardDataText>
        </ModularDashboardCard>
    );
};

TagsDashboardCard.displayName = "TagsDashboardCard";
