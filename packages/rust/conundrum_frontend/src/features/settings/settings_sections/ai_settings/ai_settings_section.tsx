import React, { type ReactNode } from "react";
import { SettingsSection } from "../settings_section";
import { GridOnLarge } from "../grid_on_large";
import { LabeledStringInput } from "#/settings/inputs/string_inputs/labeled_string_input";

export const AISettingSection = (): ReactNode => {
    return (
        <SettingsSection
            label="Artificial Intelligence"
            desc="This section is only for personalization through AI."
            className="space-y-6"
        >
            <GridOnLarge>
                <LabeledStringInput label="First Name" name="name.first" />
                <LabeledStringInput label="Last Name" name="name.last" />
            </GridOnLarge>
            <LabeledStringInput label="Profession" name="profession" />
        </SettingsSection>
    );
};

AISettingSection.displayName = "AISettingSection";
