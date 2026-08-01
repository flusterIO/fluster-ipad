import React, { type ReactNode } from "react";
import { ModularDashboardCard } from "../../modular_dashboard_card";
import { EmptyCardDataText } from "../empty_card_text";

export const SubjectsDashboardCard = (): ReactNode => {
    return (
        <ModularDashboardCard title="Subjects">
            <EmptyCardDataText>No subjects found</EmptyCardDataText>
        </ModularDashboardCard>
    );
};

SubjectsDashboardCard.displayName = "SubjectsDashboardCard";
