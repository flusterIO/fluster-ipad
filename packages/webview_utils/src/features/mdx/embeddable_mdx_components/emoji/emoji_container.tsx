import React, { type ReactNode } from 'react'
import { emojiComponentProps, type EmojiComponentProps } from './emoji_component_props'
import { type WithChildren } from '@/utils/types/utility_types'
import { cn } from '@/utils/cn'



export const EmojiContainer = ({ children, ..._props }: EmojiComponentProps & WithChildren): ReactNode => {
    const { containerClasses, inline } = emojiComponentProps.parse(_props)
    if (inline) {
        return (
            <span className={cn("inline-block [&>svg]:max-w-full [&>svg]:max-h-full", containerClasses)}>{children}</span>
        )
    }
    return (
        <div className={cn("[&>svg]:max-w-full [&>svg]:max-h-full block", containerClasses)}>{children}</div>
    )
}


EmojiContainer.displayName = "EmojiContainer"
