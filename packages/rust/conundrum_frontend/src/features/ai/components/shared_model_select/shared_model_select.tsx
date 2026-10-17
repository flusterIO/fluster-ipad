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
import React, { useState, type ReactNode } from "react";
import { ModelItem } from "./model_item";
import { useBothProviderModels } from "./use_both_provider_models";
import { useChatPageContext, useChatPageDispatch } from "#/ai/pages/ai_chat_page/chat_page_context/chat_page_context";

export const SharedModelSelect = (): ReactNode => {
    const { data: models } = useBothProviderModels();
    const { selectedModel } = useChatPageContext();
    const dispatch = useChatPageDispatch();
    const [modelSelectorOpen, setModelSelectorOpen] = useState(false);
    const getProviderKey = (source: "local" | "remote") =>
        source === "local" ? "ollama" : REMOTE_AI_PROVIDER_KEY;
    const selectedModelDetails = selectedModel
        ? models.find((m) => m.id === selectedModel.model_id && getProviderKey(m.source) === selectedModel.provider)
        : null;
    return (
        <ModelSelector onOpenChange={setModelSelectorOpen} open={modelSelectorOpen}>
            <ModelSelectorTrigger
                render={(props) => {
                    if (selectedModelDetails) {
                        return (
                            <PromptInputButton {...props}>
                                <ModelSelectorLogo
                                    provider={getProviderKey(selectedModelDetails.source)}
                                />
                                <ModelSelectorName>
                                    {selectedModelDetails.name ?? selectedModelDetails.id}
                                </ModelSelectorName>
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
                                        isSelected={
                                            selectedModel?.model_id === m.id &&
                                            selectedModel.provider === getProviderKey(m.source)
                                        }
                                        key={`${m.source}:${m.id}`}
                                        m={m}
                                        setSelected={(m) => {
                                            dispatch({
                                                type: "set-selected-model",
                                                payload: {
                                                    provider: getProviderKey(m.source),
                                                    model_id: m.id,
                                                },
                                            });
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
