import React, { type ReactNode } from "react";
import { RecentNotesCard } from "./modular_dashboard_components/dashboard_cards/recent_notes/recent_notes_card";
import { TagsDashboardCard } from "./modular_dashboard_components/dashboard_cards/taggables/tags_dashboard_card";
import { SubjectsDashboardCard } from "./modular_dashboard_components/dashboard_cards/taggables/subjects_dashboard_card";
import { TopicsDashboardCard } from "./modular_dashboard_components/dashboard_cards/taggables/topics_dashboard_card";
import { RecentBibEntriesDashboardCard } from "./modular_dashboard_components/dashboard_cards/recent_bibliography_entries/recent_bibliography_enries_dashboard_card";

export const ModularDataDashboard = (): ReactNode => {
    return (
        <div className="w-full h-full min-h-screen px-4 py-4">
            <div className="w-full flex flex-col justify-start items-start @[768px]/main:grid! @[768px]/main:grid-cols-[2fr_1fr] gap-x-4">
                <div className="w-full h-fit mb-4 @[768px]/main:h-full @[768px]/main:mb-0 grid grid-cols-1 grid-rows-[2fr_1fr] gap-y-4">
                    <RecentNotesCard />
                    <RecentBibEntriesDashboardCard />
                </div>
                <div className="w-full grid grid-cols-1 grid-rows-[2fr_1fr] max-w-full gap-y-4 min-h-[calc(100vh-2rem)]">
                    <div className="grid grid-rows-2 grid-cols-1 gap-y-4">
                        <TagsDashboardCard />
                        <SubjectsDashboardCard />
                    </div>
                    <TopicsDashboardCard />
                </div>
            </div>
        </div>
    );
};

ModularDataDashboard.displayName = "ModularDataDashboard";
