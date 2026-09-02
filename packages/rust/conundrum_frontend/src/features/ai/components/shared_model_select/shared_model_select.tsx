import { PROVIDERS, REMOTE_AI_PROVIDER_KEY } from "#/database/ai_constants";
import {
    ModelSelector,
    ModelSelectorTrigger,
    ModelSelectorLogo,
    ModelSelectorName,
    ModelSelectorContent,
    ModelSelectorInput,
    ModelSelectorList,
    ModelSelectorEmpty,
    ModelSelectorGroup,
} from "@/components/ai_elements/model_selector";
import { PromptInputButton } from "@/components/ai_elements/prompt_input";
import React, { useEffect, useState, type ReactNode } from "react";
import { ModelItem } from "./model_item";
import { useBothProviderModels } from "./use_both_provider_models";

interface SharedModelSelectProps {
    /* trigger: ReactNode; */
    selectedModelId: string;
    setSelectedModelId: (item: string) => void;
}

export const SharedModelSelect = ({
    selectedModelId,
    setSelectedModelId,
}: SharedModelSelectProps): ReactNode => {
    const { data: models } = useBothProviderModels();
    const [modelSelectorOpen, setModelSelectorOpen] = useState(false);
    const [selectedModel, setSelectedModel] = useState(
        selectedModelId
            ? models.find((m) => {
                return m.id === selectedModelId;
            })
            : null,
    );

    useEffect(() => {
        if (
            selectedModel?.id !== selectedModelId &&
            Boolean(selectedModel?.id.length)
        ) {
            // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
            setSelectedModelId(selectedModel!.id);
        }
    }, [selectedModelId, selectedModel]);

    useEffect(() => {
        setSelectedModel(
            selectedModelId
                ? models.find((m) => {
                    return m.id === selectedModelId;
                })
                : null,
        );
    }, [models, selectedModelId]);
    return (
        <ModelSelector onOpenChange={setModelSelectorOpen} open={modelSelectorOpen}>
            <ModelSelectorTrigger
                render={(props) => {
                    if (selectedModel) {
                        return (
                            <PromptInputButton {...props}>
                                <ModelSelectorLogo
                                    provider={
                                        selectedModel.source === "local"
                                            ? "ollama"
                                            : REMOTE_AI_PROVIDER_KEY
                                    }
                                />
                                <ModelSelectorName>{selectedModel.name}</ModelSelectorName>
                            </PromptInputButton>
                        );
                    } else {
                        return (
                            <PromptInputButton {...props}>
                                <ModelSelectorLogo provider={"ollama"} />
                                <ModelSelectorName>Providers</ModelSelectorName>
                            </PromptInputButton>
                        );
                    }
                }}
            />
            <ModelSelectorContent>
                <ModelSelectorInput placeholder="Search models..." />
                <ModelSelectorList>
                    <ModelSelectorEmpty>No models found.</ModelSelectorEmpty>
                    {PROVIDERS.map((chef) => (
                        <ModelSelectorGroup heading={chef.name} key={chef.key}>
                            {models
                                .filter((m) => {
                                    return chef.source === m.source;
                                })
                                .map((m) => (
                                    <ModelItem
                                        isSelected={selectedModel?.id === m.id}
                                        key={m.id}
                                        m={m}
                                        setSelected={(m) => {
                                            setSelectedModel(m);
                                            setModelSelectorOpen(false);
                                        }}
                                    />
                                ))}
                        </ModelSelectorGroup>
                    ))}
                </ModelSelectorList>
            </ModelSelectorContent>
        </ModelSelector>
    );
};

SharedModelSelect.displayName = "SharedModelSelect";
