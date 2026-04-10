import React, { type ReactNode } from 'react'
import { emojiComponentProps, type EmojiComponentProps } from './emoji_component_props'
import { type WithChildren } from '@/utils/types/utility_types'
import { cn } from '@/utils/cn'



export const EmojiContainer = ({ children, ..._props }: EmojiComponentProps & WithChildren): ReactNode => {
    const { containerClasses } = emojiComponentProps.parse(_props)
    return (
        <div className={cn("[&>svg]:max-w-full [&>svg]:max-h-full", containerClasses)}>{children}</div>
    )
}


EmojiContainer.displayName = "EmojiContainer"
