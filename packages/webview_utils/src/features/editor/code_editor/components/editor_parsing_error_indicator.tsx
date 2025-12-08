import { SplitviewEditorWebviewEvents } from '@/code_gen/typeshare/fluster_core_utilities'
import { useEventListener } from '@/state/hooks/use_event_listener'
import { XCircleIcon, XIcon } from 'lucide-react'
import React, { useState, type ReactNode } from 'react'
import { motion } from "framer-motion"


export const MdxParsingErrorIndicator = (): ReactNode => {
    const [open, setOpen] = useState(false)
    useEventListener(SplitviewEditorWebviewEvents.EmitMdxParsingSuccess, () => setOpen(false))
    useEventListener(SplitviewEditorWebviewEvents.EmitMdxParsingError, () => setOpen(true))
    if (!open) {
        return null
    }
    return (
        <motion.div
            className="mdx-error-indicator rounded-lg bg-popover text-popover-foreground absolute bottom-[2rem] relative flex flex-row justify-start items-center gap-4 min-w-[120px] max-w-[min(90vw,300px)]"
            style={{
                borderLeft: "4px solid hsl(var(--destructive))"
            }}
            initial="hide"
            animate={open ? "show" : "hide"}
            variants={{
                hide: {
                    right: -200,
                    opacity: 0
                },
                show: {
                    right: 32,
                    opacity: 1
                }
            }}
        >
            <XCircleIcon
                className="bg-destructive text-destructive-foreground"
            />
            <div className="flex flex-col justify-center items-start gap-y-2 flex-grow">
                <div className="text-lg font-bold text-popover-foreground">Error</div>
                <div className="text-thin text-popover-foreground">An error occurred while attempting to parse this mdx content. Please verify your syntax.</div>
            </div>
            <XIcon
                className="absolute top-4 right-4 cursor-pointer"
                onClick={() => setOpen(false)}
            />
        </motion.div>
    )
}


MdxParsingErrorIndicator.displayName = "MdxParsingErrorIndicator"
