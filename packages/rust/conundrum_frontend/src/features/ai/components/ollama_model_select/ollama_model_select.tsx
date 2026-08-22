import {
    ModelSelector,
    ModelSelectorContent,
    ModelSelectorDialog,
    ModelSelectorInput,
    ModelSelectorList,
    ModelSelectorTrigger,
} from "@/components/ai_elements/model_selector";
import React, { type ReactNode } from "react";

interface OllamaModelSelectProps {
    children: ReactNode;
}

export const OllamaModelSelect = ({
    children,
}: OllamaModelSelectProps): ReactNode => {
    return (
        <ModelSelector>
            <ModelSelectorTrigger>{children}</ModelSelectorTrigger>
            <ModelSelectorDialog>
                <ModelSelectorContent>
                    <ModelSelectorInput />
                    <ModelSelectorList></ModelSelectorList>
                </ModelSelectorContent>
            </ModelSelectorDialog>
        </ModelSelector>
    );
};

OllamaModelSelect.displayName = "OllamaModelSelect";
