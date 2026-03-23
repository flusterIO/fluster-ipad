import { emphasisOptions } from '#/mdx/embeddable_mdx_components/schemas/emphasis_schema'
import React, { type ReactNode } from 'react'
import { Hl } from '../../../mdx/embeddable_mdx_components/hl/hl'

export const InContentHighlightDocsDemo = (): ReactNode => {
    return (
        <div className="w-full flex flex-col justify-start items-start gap-y-1">
            {emphasisOptions.map((opt) => {
                const props = {
                    [opt]: true
                }
                return (
                    <div>This is <Hl {...props}>{opt}</Hl> highlighted</div>
                )
            })}
        </div>
    )
}


InContentHighlightDocsDemo.displayName = "HighlightDocsDemo"
