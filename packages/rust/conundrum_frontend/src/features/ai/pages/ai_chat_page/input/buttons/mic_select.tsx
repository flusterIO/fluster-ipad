import {
    MicSelectorProps,
    MicSelectorTrigger,
    MicSelector as MS,
    MicSelectorList,
    MicSelectorItem,
    MicSelectorContent,
    MicSelectorLabel,
} from "@/components/ai_elements/mic_selector";
import { PromptInputButton } from "@/components/ai_elements/prompt_input";
import { MicIcon } from "lucide-react";
import React, { useState, type ReactNode } from "react";

export const MicSelector = (): ReactNode => {
    const [value, setValue] = useState<string | null>(null);
    return (
        <MS
            value={value ?? ""}
            onValueChange={(val) => {
                setValue(val ?? null);
            }}
        >
            <MicSelectorTrigger>
                <PromptInputButton tooltip={"Voice Input"}>
                    <MicIcon size={16} />
                </PromptInputButton>
            </MicSelectorTrigger>
            <MicSelectorContent
                className="w-full"
                popoverOptions={{
                    align: "start",
                    className: "w-full min-w-fit p-0",
                }}
            >
                <MicSelectorList className="w-full min-w-fit p-0">
                    {(devices) => {
                        return devices.map((k) => {
                            return (
                                <MicSelectorItem
                                    value={k.deviceId}
                                    className="text-nowrap w-full"
                                >
                                    <MicSelectorLabel device={k}>Microphones</MicSelectorLabel>
                                </MicSelectorItem>
                            );
                        });
                    }}
                </MicSelectorList>
            </MicSelectorContent>
        </MS>
    );
};

MicSelector.displayName = "MicSelector";
