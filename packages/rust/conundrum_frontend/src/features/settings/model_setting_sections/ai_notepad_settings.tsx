import { SettingsSectionTitle } from "#/ui/typography/settings_section_title";
import React, { type ComponentProps, type ReactNode } from "react";
import { LabeledTextAreaInput } from "../inputs/string_inputs/labeled_text_area";
import { InfoIcon } from "lucide-react";
import {
    HoverCard,
    HoverCardContent,
    HoverCardTrigger,
} from "@/components/shad/hover-card";

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
                            <HoverCard>
                                <HoverCardTrigger>
                                    <InfoIcon className="inline w-4 h-4 mx-2" />
                                </HoverCardTrigger>
                                <HoverCardContent>
                                    <h4 className="text-lg font-semibold">Work in progress</h4>
                                    <div>
                                        Work is being undertaken to enable this UI for all of the
                                        dozens of models contained within the Conundrum database,
                                        but this will take time.
                                    </div>
                                    <div>
                                        Please consider supporting the project or contributing to
                                        the project directly if you would like to speed things up.
                                    </div>
                                </HoverCardContent>
                            </HoverCard>
                        </>
                    )
                }
            >
                AI Notepad
            </SettingsSectionTitle>
            <LabeledTextAreaInput
                name="ai.notes"
                label="Tell AI about this instance"
                {...notesProps}
            />
        </>
    );
};

AINotepadSettings.displayName = "AINotepadSettings";
