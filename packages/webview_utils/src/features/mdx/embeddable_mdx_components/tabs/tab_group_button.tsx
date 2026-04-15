import React, { type ReactNode } from 'react'
import { type EmbeddableTabItem } from './types'
import { cn } from '@/utils/cn'
import { motion } from 'framer-motion'
import { useEmbeddableTabGroupContext, useEmbeddableTabGroupDispatch } from './state/embeddable_tab_group_context'



interface TabGroupButtonProps {
    item: EmbeddableTabItem
    idx: number
    isLast?: boolean
}

export const TabGroupButton = ({ item, isLast, idx }: TabGroupButtonProps): ReactNode => {
    const dispatch = useEmbeddableTabGroupDispatch()
    const { focusedIndex, activeTabClasses, subtle, lastIndex } = useEmbeddableTabGroupContext()

    if (!subtle) {
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

    return (
        <button
            className={cn("flex flex-col justify-center items-center gap-0 flex-grow-1 pt-1 pb-0 border cursor-pointer", idx === 0 && "rounded-tl", isLast && "rounded-tr")}>
            <div
                onClick={() => {
                    dispatch({
                        type: "setFocusedTabIndex",
                        payload: idx
                    })
                }}
                aria-disabled={idx === focusedIndex}
                className={"px-3 text-nowrap text-left"}>{item.label}</div>
            <div className="w-full h-1">
                <motion.div
                    className={cn("w-full h-1 bg-transparent", idx === focusedIndex && activeTabClasses)}
                    animate={idx === focusedIndex ? "show" : "hide"}
                    variants={{
                        show: {
                            scaleX: 1,
                            originX: (lastIndex !== null && lastIndex < focusedIndex) ? "left" : "right"
                        },
                        hide: {
                            scaleX: 0,
                            originX: (lastIndex !== null && lastIndex < focusedIndex) ? "left" : "right"
                        }
                    }}
                />
            </div>
        </button>
    )
}


TabGroupButton.displayName = "TabGroupButton"
