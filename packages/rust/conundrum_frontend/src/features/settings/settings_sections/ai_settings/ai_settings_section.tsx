import React, { type ReactNode } from "react";
import { SettingsSection } from "../settings_section";
import { GridOnLarge } from "../grid_on_large";
import { LabeledStringInput } from "#/settings/inputs/string_inputs/labeled_string_input";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/shad/card";
import { DateTimeInput } from "#/settings/inputs/date_time/time_input";

const minutesToMilliseconds = (mins: number): number => {
    return mins * 60 * 1000
}

export const AISettingSection = (): ReactNode => {
    return (
        <SettingsSection
            label="Artificial Intelligence"
            desc="This section is only for personalization through AI."
            className="space-y-6"
        >
            <Card
            >
                <CardHeader>
                    <CardTitle>Personal Details</CardTitle>
                </CardHeader>
                <CardContent>
                    <GridOnLarge>
                        <LabeledStringInput label="First Name" name="name.first" />
                        <LabeledStringInput label="Last Name" name="name.last" />
                    </GridOnLarge>
                    <LabeledStringInput label="Profession" name="profession" />
                </CardContent>
            </Card>
            <Card>
                <CardHeader>
                    <CardTitle>Daily Chat</CardTitle>
                    <CardDescription>Control the time at which the daily chat expires.</CardDescription>
                </CardHeader>
                <CardContent>
                    <DateTimeInput
                        hideDate
                        name="daily_chat.expires_time"
                        timeLabel="Expires at"
                        formatter={(d) => {
                            const offset = new Date().getTimezoneOffset();
                            const t = d.valueOf() + minutesToMilliseconds(offset);
                            const midnight = d.startOf("day").valueOf();
                            return t - midnight;
                        }}
                    />
                </CardContent>
            </Card>
        </SettingsSection>
    );
};

AISettingSection.displayName = "AISettingSection";
