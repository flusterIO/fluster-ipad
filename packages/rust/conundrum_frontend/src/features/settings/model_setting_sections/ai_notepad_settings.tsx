import { SettingsSectionTitle } from "#/ui/typography/settings_section_title";
import React, { type ComponentProps, type ReactNode } from "react";
import { LabeledTextAreaInput } from "../inputs/string_inputs/labeled_text_area";

export const AINotepadSettings = ({
    notesProps,
    desc,
}: {
    notesProps?: Omit<ComponentProps<typeof LabeledTextAreaInput>, "name">;
    desc?: ReactNode;
}): ReactNode => {
    return (
        <>
            <SettingsSectionTitle
                desc={
                    desc ?? (
                        <>
                            This is where you communicate with AI about this specific instance
                            of this model. AI will be given an opportunity to take notes,
                            creating a sort of memory of it's own, and with each request, will
                            get to know your knowledge base better.
                        </>
                    )
                }
            >
                AI Notepad
            </SettingsSectionTitle>
            <LabeledTextAreaInput
                name="notes"
                label="Tell AI about this instance"
                {...notesProps}
            />
        </>
    );
};

AINotepadSettings.displayName = "AINotepadSettings";
