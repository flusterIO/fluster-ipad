import React, { useEffect, type ReactNode } from 'react'
import { useNoteSummary } from './use_note_summary'
import { AIGeneratedContainer } from '#/ai/presentation/ai_generated_card'
import { aiNoteSummaryProps, type AINoteSummaryProps } from './ai_note_summary_props'
import { cn } from '../../../../core/utils/cn'
import { useDispatch } from 'react-redux'
import { resetSummary } from '#/webview_global_state/note_details_state/note_details_slice'


export const AiNoteSummary = (_props: AINoteSummaryProps): ReactNode => {
    const { generate, containerClasses } = aiNoteSummaryProps.parse(_props)
    const dispatch = useDispatch()

    useEffect(() => {
        if (generate) {
            dispatch(resetSummary())
        }
    }, [generate])

    const summary = useNoteSummary()

    return (
        <AIGeneratedContainer
            className={cn("block w-full max-w-[1080px]", containerClasses)}
        >
            {summary?.content ?? "No summary found, generating..."}
        </AIGeneratedContainer>
    )
}


AiNoteSummary.displayName = "AiNoteSummary"
