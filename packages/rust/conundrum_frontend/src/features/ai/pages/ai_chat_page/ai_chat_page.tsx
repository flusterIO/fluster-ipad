import {
    PromptInput,
    PromptInputBody,
    PromptInputButton,
    PromptInputFooter,
    PromptInputSubmit,
    PromptInputTextarea,
    PromptInputTools,
} from "@/components/ai_elements/prompt_input";
import { randomFromArray } from "@/utils/array_utils";
import { MicIcon, PaperclipIcon } from "lucide-react";
import React, { type ReactNode } from "react";

export const GeneralAIChatPage = (): ReactNode => {
    return (
        <div className="w-full h-screen flex flex-col justify-between items-center">
            <div className="grow w-full overflow-x-hidden overflow-y-auto min-scrollbar"></div>
            <PromptInput
                onSubmit={(val) => {
                    console.log("val: ", val);
                }}
                className="mb-4 mx-4 w-270 max-w-[calc(100%-4rem)]"
            >
                <PromptInputBody>
                    <PromptInputTextarea
                        placeholder={randomFromArray([
                            "How can I help?",
                            "What are we working on today?",
                            "What would you like to know?",
                            "Anything I can help with?",
                            "I'm here for whatever you need",
                            "Let's get to work...",
                            "Let's change the world...",
                        ])}
                    />
                </PromptInputBody>
                <PromptInputFooter>
                    <PromptInputTools>
                        <PromptInputButton>
                            <PaperclipIcon size={16} />
                        </PromptInputButton>
                        <PromptInputButton tooltip={"Voice Input"}>
                            <MicIcon size={16} />
                        </PromptInputButton>
                    </PromptInputTools>
                    <PromptInputSubmit />
                </PromptInputFooter>
            </PromptInput>
        </div>
    );
};

GeneralAIChatPage.displayName = "GeneralAIChatPage";
