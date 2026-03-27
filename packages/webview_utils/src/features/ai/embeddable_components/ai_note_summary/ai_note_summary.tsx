import React, { type ReactNode } from 'react'
import { useNoteSummary } from './use_note_summary'
import { AIGeneratedContainer } from '#/ai/presentation/ai_generated_card'

export const AiNoteSummary = (): ReactNode => {
    const summary = useNoteSummary()
    return (
        <AIGeneratedContainer
            className="block w-full max-w-[1080px]"
        >
            {summary?.content ?? "No summary found, generating..."}
        </AIGeneratedContainer>
    )
}


AiNoteSummary.displayName = "AiNoteSummary"
