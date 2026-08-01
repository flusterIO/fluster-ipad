import React, { type ReactNode } from "react";
import { AISettingSection } from "./settings_sections/ai_settings/ai_settings_section";

export const MainSettingsPage = (): ReactNode => {
    return (
        <div className="@container/settings w-full max-w-270 mx-auto px-6">
            <AISettingSection />
        </div>
    );
};

MainSettingsPage.displayName = "MainSettingsPage";
