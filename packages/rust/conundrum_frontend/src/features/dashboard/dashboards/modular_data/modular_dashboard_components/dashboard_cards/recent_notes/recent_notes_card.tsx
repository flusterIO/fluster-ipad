import React, { type ReactNode } from "react";
import { ModularDashboardCard } from "../../modular_dashboard_card";
import { EmptyCardDataText } from "../empty_card_text";

export const RecentNotesCard = (): ReactNode => {
    return (
        <ModularDashboardCard title="Recent Notes">
            <EmptyCardDataText>No notes to show</EmptyCardDataText>
        </ModularDashboardCard>
    );
};

RecentNotesCard.displayName = "RecentNotesCard";
