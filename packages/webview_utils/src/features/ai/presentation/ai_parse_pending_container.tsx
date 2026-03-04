import React, { type ReactNode } from 'react'
import { FlusterAiParseButton } from "./ai_parse_button"


interface AiParsePendingContainerProps {
    children: ReactNode
}

export const AiParsePendingContainer = ({ children }: AiParsePendingContainerProps): ReactNode => {
    return (
        <div
            className="w-full max-w-[min(1080px,90%)] p-4 border border-fd-ring bg-fd-card rounded text-card-foreground [&_*]:text-card-foreground @container/aiParsePendingContainer relative"
        >
            <div>
                {children}
            </div>
            <div className="w-full flex flex-row justify-end items-center">
                <div className="invisible @[540px]/aiParsePendingContainer:visible absolute bottom-3 left-3 text-sm opacity-60">
                    See docs for supported actions
                </div>
                <FlusterAiParseButton />
            </div>
        </div>
    )
}


AiParsePendingContainer.displayName = "AiParsePendingContainer"
