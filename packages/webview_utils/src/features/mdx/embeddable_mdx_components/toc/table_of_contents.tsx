import { type MarkdownHeadingStringifiedResult } from '@/code_gen/typeshare/conundrum'
import React, { type ReactNode } from 'react'
import { InlineToc } from './toc_views/inline/inline_toc'
import { tocProps } from './toc_props'



interface TableOfContentsProps {
    toc: MarkdownHeadingStringifiedResult[]
}

export const TableOfContents = ({ toc, ..._props }: TableOfContentsProps): ReactNode => {
    // TODO: Allow an option to be passed to the component to enable diferent views once the Conundrum enum is supported.
    const props = tocProps.parse(_props)
    return (
        <InlineToc toc={toc} initiallyExpanded={props.expanded ?? false} />
    )
}


TableOfContents.displayName = "TableOfContents"
