import React, { useRef, type ReactNode } from 'react'
import { type AiErrorState } from './types'
import { DynamicIcon } from 'lucide-react/dynamic'


interface AiParsePendingErrorStatusContainerProps {
    error: AiErrorState
}

export const AiParsePendingErrorStatusContainer = ({ error }: AiParsePendingErrorStatusContainerProps): ReactNode => {
    const iconRef = useRef<HTMLDivElement>(null)
    return (
        <div className="border border-emphasis-warn/40! rounded w-fit max-w-[min(1080px,90%)] flex flex-col justify-center items-center px-4 py-3 bg-card mx-auto">
            <div className="w-fit h-fit rounded-[100%] bg-emphasis-warn p-1"
                ref={iconRef}
            >
                <DynamicIcon name='radiation' className="w-12 h-12 text-emphasis-warn-foreground" />
            </div>
            <div className="font-bold">AI Status Error</div>
            <div className="w-full text-center">
                {error.message()}
            </div>
        </div>
    )
}


AiParsePendingErrorStatusContainer.displayName = "AiParsePendingErrorStatusContainer"
