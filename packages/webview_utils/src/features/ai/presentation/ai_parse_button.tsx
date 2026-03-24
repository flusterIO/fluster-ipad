import React from 'react'
import { ChevronRightIcon } from 'lucide-react'
import { cn } from '@/utils/cn'


export const FlusterAiParseButton = ({
    onClick,
    classes = {},
}: {
    onClick: () => void,
    classes?: {
        button?: string
        innerContainer?: string
    }
}) => {
    return (
        <button className={cn("relative inline-flex h-12 overflow-hidden rounded-full p-[1px] focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-2 focus:ring-offset-slate-50 force-background", classes.button)} onClick={onClick}>
            <span className="absolute inset-[-1000%] animate-[spin_2s_linear_infinite] bg-[conic-gradient(from_90deg_at_50%_50%,#0ba5e9_0%,#393BB2_50%,#0ba5e9_100%)]" />
            <div className={cn("inline-flex flex-row flex-nowrap h-full w-full cursor-pointer items-center justify-center rounded-full pr-4 pl-5 py-1 text-sm font-medium text-foreground backdrop-blur-3xl force-background", classes.innerContainer)}>
                <span>
                    Generate
                </span>
                <ChevronRightIcon
                    className="w-4 h-4 min-w-4 min-h-4"
                />
            </div>
        </button>
    )
}



FlusterAiParseButton.displayName = "FlusterAiParseButton"
