import { type EditorCitation } from '@/code_gen/typeshare/fluster_core_utilities'
import { cn } from '@/utils/cn'
import { ChevronRight } from 'lucide-react'
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
    onClick?: () => void
}

export const BibliographyEntryComponent = ({ entry, onClick, className, idx }: BibliographyEntryComponentProps): ReactNode => {
    if (idx) {
        return (
            <div className={cn("grid grid-cols-[64px_1fr] w-full", idx.classes?.container)}>
                <div className={cn("text-sm place-self-center", idx.classes?.index)}>{idx.index}</div>
                <a href={entry.url} onClick={entry.url ? undefined : onClick} className={cn("not-prose", className, idx.classes?.content)} dangerouslySetInnerHTML={{ __html: entry.html }} />
            </div>
        )
    }
    if (entry.url) {
        return (
            <div
                className={cn("grid grid-cols-[1fr_auto] w-full px-3 pl-2 rounded-lg cursor-pointer", className)}
            >
                <div
                    className={cn("not-prose w-full text-sm", entry.url ? "cursor-pointer" : "cursor-default")} dangerouslySetInnerHTML={{ __html: entry.html }} onClick={onClick}
                />
                <a
                    href={entry.url}
                    className="w-full h-full flex flex-col justify-center items-center cursor-pointer hover:opacity-100 opacity-70 transition-all duration-300"
                >
                    <ChevronRight className="place-self-center w-5 h-5 mr-2 my-2 ml-4 text-fd-card-foreground" />
                </a>
            </div>
        )
    }
    return (
        <a
            role="button"
            className={cn("w-full w-full px-3 pl-2 rounded-lg text-sm [&_*]:text-sm cursor-pointer")}
            dangerouslySetInnerHTML={{ __html: entry.html }}
            onClick={onClick}
        />
    )
}


BibliographyEntryComponent.displayName = "BibliographyEntryComponent"
