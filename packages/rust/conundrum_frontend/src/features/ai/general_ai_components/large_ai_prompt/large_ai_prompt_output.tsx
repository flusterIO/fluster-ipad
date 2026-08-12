import React, { type ReactNode } from 'react'
import { type LargeAIPromptViewMode } from './large_ai_prompt_text_input'
import { motion } from 'framer-motion'



interface LargeAIPromptOutputProps {
    content: ReactNode
    viewMode: LargeAIPromptViewMode
}

export const LargeAIPromptOutput = ({ content }: LargeAIPromptOutputProps): ReactNode => {
    return (
        <motion.div
            className="overflow-x-hidden overflow-y-auto"
            initial={{
                opacity: 0
            }}
            animate={{
                opacity: 1
            }}
            exit={{
                opacity: 0,
                scale: 0
            }}
        >{content}</motion.div>
    )
}


LargeAIPromptOutput.displayName = "LargeAIPromptOutput"
