import { type EditorCitation } from '@/code_gen/typeshare/fluster_core_utilities'
import { cn } from '@/utils/cn'
import React, { type ReactNode } from 'react'



interface BibliographyEntryComponentProps {
    entry: EditorCitation
    className?: string
    idx?: {
        index: number
        classes?: {
            container?: string
            /**
             * index is 0 based at this point.
             */
            index?: string
            content?: string
        }
    }
    /**
     * onClick is only called when url is void
     */
    onClickIfNoUrl?: () => void
}

export const BibliographyEntryComponent = ({ entry, onClickIfNoUrl, className, idx }: BibliographyEntryComponentProps): ReactNode => {
    if (idx) {
        return (
            <div className={cn("grid grid-cols-[64px_1fr] w-full", idx.classes?.container)}>
                <div className={cn("text-sm place-self-center", idx.classes?.index)}>{idx.index}</div>
                <a href={entry.url} onClick={entry.url ? undefined : onClickIfNoUrl} className={cn("not-prose", className, idx.classes?.content)} dangerouslySetInnerHTML={{ __html: entry.html }} />
            </div>
        )
    }
    return (
        <a href={entry.url} className={cn("not-prose w-full", className)} dangerouslySetInnerHTML={{ __html: entry.html }} onClick={entry.url ? undefined : onClickIfNoUrl} />
    )
}


BibliographyEntryComponent.displayName = "BibliographyEntryComponent"
