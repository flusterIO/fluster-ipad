import { StringSetting } from "#/settings/inputs/string_setting";
import React, { type ReactNode } from "react";
import { GridOnLarge } from "../grid_on_large";
import { SettingsFieldDescription } from "../settings_field_description";
import { SettingsSection } from "../settings_section";
import { DescriptiveItemsSelect } from "#/settings/inputs/select/descriptive_items_select/descriptive_items_select";

export const NotificationSettings = (): ReactNode => {
    return (
        <SettingsSection label="Notification & Logging">
            <DescriptiveItemsSelect
                label="Notification Storage"
                name="notifications"
                options={[
                    {
                        label: "Store & Embed",
                        desc: "Store logs, and embed vectors.",
                        value: "store-embed",
                        id: "store-embed",
                    },
                    {
                        label: "Store",
                        desc: "Store logs, but do not generate vectors.",
                        value: "store",
                        id: "store",
                    },
                ]}
            />
        </SettingsSection>
    );
};

NotificationSettings.displayName = "NotificationSettings";
