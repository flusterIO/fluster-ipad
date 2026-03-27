import React, { type ReactNode } from 'react'
import { useNoteSummary } from './use_note_summary'
import { AIGeneratedContainer } from '#/ai/presentation/ai_generated_card'
import { aiNoteSummaryProps, type AINoteSummaryProps } from './ai_note_summary_props'
import { cn } from '../../../../core/utils/cn'


export const AiNoteSummary = (_props: AINoteSummaryProps): ReactNode => {
    const props = aiNoteSummaryProps.parse(_props)

    const summary = useNoteSummary(props.generate)

    return (
        <AIGeneratedContainer
            className={cn("block w-full max-w-[1080px]", props.containerClasses)}
        >
            {summary?.content ?? "No summary found, generating..."}
        </AIGeneratedContainer>
    )
}


AiNoteSummary.displayName = "AiNoteSummary"
