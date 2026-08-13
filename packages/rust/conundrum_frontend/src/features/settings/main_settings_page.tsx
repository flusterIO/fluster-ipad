import React, { type ReactNode } from "react";
import { AISettingSection } from "./settings_sections/ai_settings/ai_settings_section";
import { NotificationSettings } from "./settings_sections/notifications/notification_settings";

export const MainSettingsPage = (): ReactNode => {
    return (
        <div className="@container/settings w-full max-w-270 mx-auto px-6">
            <AISettingSection />
            <NotificationSettings />
        </div>
    );
};

MainSettingsPage.displayName = "MainSettingsPage";
