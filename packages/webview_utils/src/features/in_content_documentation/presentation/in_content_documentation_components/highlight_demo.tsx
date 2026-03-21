import { MdxContent } from '#/mdx/components/mdx_content'
import { emphasisOptions } from '#/mdx/embeddable_mdx_components/schemas/emphasis_schema'
import React, { type ReactNode } from 'react'

export const HighlightDocsDemo = (): ReactNode => {
    return (
        <MdxContent
            mdx={emphasisOptions.map((opt) => {
                return `- <Hl ${opt}>This is ${opt} highlighted</Hl>`
            }).join("\n")}
        />
    )
}


HighlightDocsDemo.displayName = "HighlightDocsDemo"
