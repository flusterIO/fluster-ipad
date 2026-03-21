import { BlockQuote } from '@/shared_components/typography/block_quote'
import React, { type ReactNode } from 'react'



interface AutoInsertedBlockQuoteProps {
    children: ReactNode
}

export const AutoInsertedBlockQuote = ({ children }: AutoInsertedBlockQuoteProps): ReactNode => {
    return (
        <BlockQuote>
            {children}
        </BlockQuote>

    )
}


AutoInsertedBlockQuote.displayName = "AutoInsertedBlockQuote"
