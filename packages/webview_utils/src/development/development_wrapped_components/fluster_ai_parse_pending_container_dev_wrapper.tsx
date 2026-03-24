import { FlusterAiParsePendingContainer } from '#/ai/presentation/ai_parse_pending_container'
import { type ParsedCodeBlock } from '@/code_gen/typeshare/conundrum'
import React, { type ReactNode } from 'react'



interface AiParsePhase1DevContainerProps {
    data?: Partial<ParsedCodeBlock>
}

export const AiParsePhase1DevContainer = ({ data = {} }: AiParsePhase1DevContainerProps): ReactNode => {
    return (
        <FlusterAiParsePendingContainer
            adfcbzadfjadfadfkhllakadf
            res={{
                content: data.content ?? "Can you summarize this note please?",
                full_match: data.full_match ?? "```fluster-ai\nCan you help summarize this note please?\n```",
                language: data.language ?? "fluster-ai",
                meta_data: data.meta_data ?? undefined

            }}
        />
    )
}


AiParsePhase1DevContainer.displayName = "AiParsePhase1DevContainer"
