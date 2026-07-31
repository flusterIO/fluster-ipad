import React, { type ReactNode } from "react";
import { RecentNotesCard } from "./modular_dashboard_components/dashboard_cards/recent_notes/recent_notes_card";
import { TaggablesDashboardCard } from "./modular_dashboard_components/dashboard_cards/taggables/taggables_card_container";

export const ModularDataDashboard = (): ReactNode => {
    return (
        <div className="w-full h-full min-h-screen px-4 py-4 overflow-x-hidden overflow-y-auto min-scrollbar">
            <div
                className="grid grid-cols-1 md:grid-cols-[2fr_1fr] gap-4"
            >
                <RecentNotesCard />
                <TaggablesDashboardCard />
            </div>
        </div>
    );
};

ModularDataDashboard.displayName = "ModularDataDashboard";
