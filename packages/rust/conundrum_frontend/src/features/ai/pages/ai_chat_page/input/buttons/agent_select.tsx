import { SharedModelSelect } from "#/ai/components/shared_model_select/shared_model_select";
import { PromptInputButton } from "@/components/ai_elements/prompt_input";
import React, { type ReactNode } from "react";

export const AgentSelectInputButton = (): ReactNode => {
    return (
        <PromptInputButton className="mx-3">
            <SharedModelSelect />
        </PromptInputButton>
    );
};

AgentSelectInputButton.displayName = "AgentSelectInputButton";
