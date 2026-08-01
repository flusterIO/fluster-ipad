import React, { type ReactNode } from "react";
import { SettingsSection } from "../settings_section";
import { StringSetting } from "#/settings/inputs/string_setting";
import { GridOnLarge } from "../grid_on_large";
import { SettingsFieldDescription } from "../settings_field_description";

export const AISettingSection = (): ReactNode => {
    return (
        <SettingsSection label="Artificial Intelligence">
            <GridOnLarge>
                <StringSetting label="First Name" settingKey="first_name" />
                <StringSetting label="Last Name" settingKey="last_name" />
            </GridOnLarge>
            <SettingsFieldDescription>
                Your name is used only for AI customization.
            </SettingsFieldDescription>
        </SettingsSection>
    );
};

AISettingSection.displayName = "AISettingSection";
