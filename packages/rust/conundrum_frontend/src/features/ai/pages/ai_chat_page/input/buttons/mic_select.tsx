import { SpeechInput } from "@/components/ai_elements/speech_input";
import {
    MicSelectorTrigger,
    MicSelector as MS,
    MicSelectorList,
    MicSelectorItem,
    MicSelectorContent,
    MicSelectorLabel,
    useAudioDevices,
} from "@/components/ai_elements/mic_selector";
import { usePromptInputController } from "@/components/ai_elements/prompt_input";
import { MicIcon } from "lucide-react";
import React, { useEffect, useState, type ReactNode } from "react";

export const MicSelector = (): ReactNode => {
    const [value, setValue] = useState<string | undefined>();
    const { devices } = useAudioDevices();
    const { textInput } = usePromptInputController();

    useEffect(() => {
        if (!value && devices.length > 0) {
            setValue(
                devices.find((device) => device.deviceId === "default")?.deviceId ??
                devices[0]?.deviceId,
            );
        }
    }, [devices, value]);

    const addTranscription = (transcript: string) => {
        const existingText = textInput.value.trimEnd();
        textInput.setInput(
            existingText ? `${existingText} ${transcript}` : transcript,
        );
    };

    return (
        <>
            <MS value={value} onValueChange={setValue}>
                <MicSelectorTrigger aria-label="Choose microphone" className="w-auto!">
                    <MicIcon size={16} />
                </MicSelectorTrigger>
                <MicSelectorContent
                    className="w-full"
                    popoverOptions={{ align: "start", className: "w-full min-w-fit p-0" }}
                >
                    <MicSelectorList className="w-full min-w-fit p-0">
                        {(availableDevices) =>
                            availableDevices.map((device) => (
                                <MicSelectorItem
                                    key={device.deviceId}
                                    value={device.deviceId}
                                    className="text-nowrap w-full"
                                >
                                    <MicSelectorLabel device={device} />
                                </MicSelectorItem>
                            ))
                        }
                    </MicSelectorList>
                </MicSelectorContent>
            </MS>
            <SpeechInput
                aria-label="Start voice input"
                audioDeviceId={value}
                onTranscriptionChange={addTranscription}
                title="Dictate into message"
            />
        </>
    );
};

MicSelector.displayName = "MicSelector";
