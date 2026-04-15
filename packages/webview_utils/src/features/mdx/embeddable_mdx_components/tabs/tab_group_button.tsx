import React, { type ReactNode } from 'react'
import { type EmbeddableTabItem } from './types'
import { cn } from '@/utils/cn'
import { useEmbeddableTabGroupContext, useEmbeddableTabGroupDispatch } from './state/embeddable_tab_group_context'



interface TabGroupButtonProps {
    item: EmbeddableTabItem
    idx: number
    isLast?: boolean
}

export const TabGroupButton = ({ item, isLast, idx }: TabGroupButtonProps): ReactNode => {
    const dispatch = useEmbeddableTabGroupDispatch()
    const { focusedIndex, activeTabClasses } = useEmbeddableTabGroupContext()


    return (
        <button
            onClick={() => {
                dispatch({
                    type: "setFocusedTabIndex",
                    payload: idx
                })
            }}
            aria-disabled={idx === focusedIndex}
            className={cn("px-2 flex-grow-1 px-3 py-1 border cursor-pointer text-nowrap text-left transition-colors duration-300", idx === 0 && "rounded-tl", isLast && "rounded-tr", idx === focusedIndex && activeTabClasses)}>{item.label}</button>
    )
}


TabGroupButton.displayName = "TabGroupButton"
