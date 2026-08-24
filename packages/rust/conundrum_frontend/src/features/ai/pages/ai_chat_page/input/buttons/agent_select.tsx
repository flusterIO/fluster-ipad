import { OllamaModelSelect } from '#/ai/components/ollama_model_select/ollama_model_select'
import { PromptInputButton } from '@/components/ai_elements/prompt_input'
import { PersonStandingIcon } from 'lucide-react'
import React, { type ReactNode } from 'react'

export const AgentSelectInputButton = (): ReactNode => {
    return (
        <PromptInputButton>
            <OllamaModelSelect>
                <PersonStandingIcon />
            </OllamaModelSelect>
        </PromptInputButton>
    )
}


AgentSelectInputButton.displayName = "AgentSelectInputButton"
