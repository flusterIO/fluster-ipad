import { PROVIDERS } from "#/database/ai_constants";
import { type ModelDescription } from "#/database/db_utility_types/agent";
import {
    ModelSelectorItem,
    ModelSelectorLogo,
    ModelSelectorName,
    ModelSelectorLogoGroup,
} from "@/components/ai_elements/model_selector";
import { CheckIcon } from "lucide-react";
import React from "react";

export interface ModelItemProps {
    m: ModelDescription & { source: "local" | "remote" };
    isSelected: boolean;
    setSelected: (
        itemId: ModelDescription & { source: "local" | "remote" },
    ) => void;
}

export const ModelItem = ({ m, isSelected, setSelected }: ModelItemProps) => {
    return (
        <ModelSelectorItem
            onSelect={() => {
                setSelected(m);
            }}
            value={m.id}
        >
            <ModelSelectorLogo provider={"ollama"} />
            <ModelSelectorName>{m.name}</ModelSelectorName>
            <ModelSelectorLogoGroup>
                {PROVIDERS.map((provider) => (
                    <ModelSelectorLogo key={provider.key} provider={provider.key} />
                ))}
            </ModelSelectorLogoGroup>
            {isSelected ? (
                <CheckIcon className="ml-auto size-4 text-foreground" />
            ) : (
                <div className="ml-auto size-4 text-foreground" />
            )}
        </ModelSelectorItem>
    );
};

ModelItem.displayName = "ModelItem";
