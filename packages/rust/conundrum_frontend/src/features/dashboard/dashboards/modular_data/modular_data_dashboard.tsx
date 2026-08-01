import React, { type ReactNode } from "react";
import { RecentNotesCard } from "./modular_dashboard_components/dashboard_cards/recent_notes/recent_notes_card";
import { TagsDashboardCard } from "./modular_dashboard_components/dashboard_cards/taggables/tags_dashboard_card";
import { SubjectsDashboardCard } from "./modular_dashboard_components/dashboard_cards/taggables/subjects_dashboard_card";
import { TopicsDashboardCard } from "./modular_dashboard_components/dashboard_cards/taggables/topics_dashboard_card";

export const ModularDataDashboard = (): ReactNode => {
    return (
        <div className="w-full h-full min-h-screen px-4 py-4 overflow-x-hidden overflow-y-auto min-scrollbar">
            <div className="w-full min-h-screen flex flex-col justify-start items-start @lg/settings:grid @lg/settings:grid-cols-[2fr_1fr]">
                <div className="w-full grid grid-cols-1">
                    <RecentNotesCard />
                </div>
                <div className="w-full flex flex-col justify-center items-center min-h-screen max-w-full">
                    <TagsDashboardCard />
                    <SubjectsDashboardCard />
                    <TopicsDashboardCard />
                </div>
            </div>
        </div>
    );
};

ModularDataDashboard.displayName = "ModularDataDashboard";
