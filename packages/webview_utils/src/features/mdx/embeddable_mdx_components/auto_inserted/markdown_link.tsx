import React, { type ReactNode } from 'react'
import { type MarkdownLinkResult } from "@/code_gen/typeshare/conundrum"



interface AutoInsertedMarkdownLinkProps {
    data: MarkdownLinkResult
}

export const AutoInsertedMarkdownLink = ({ data }: AutoInsertedMarkdownLinkProps): ReactNode => {
    return (
        <a href={data.url}>{data.text}</a>
    )
}


AutoInsertedMarkdownLink.displayName = "AutoInsertedMarkdownLink"
