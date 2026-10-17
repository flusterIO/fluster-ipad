import { SharedModelSelect } from "#/ai/components/shared_model_select/shared_model_select";
import React, { type ReactNode } from "react";

export const AgentSelectInputButton = (): ReactNode => {
    return <SharedModelSelect />;
};

AgentSelectInputButton.displayName = "AgentSelectInputButton";
