import { type EditorCitation } from '@/code_gen/typeshare/fluster_core_utilities'
import { cn } from '@/utils/cn'
import { ChevronRight } from 'lucide-react'
import React, { type ReactNode } from 'react'



interface BibliographyEntryComponentProps {
    entry: EditorCitation
    className?: string
    /**
     * onClick is only called when url is void
     */
    onClick?: () => void
}

export const BibliographyEntryComponent = ({ entry, onClick, className }: BibliographyEntryComponentProps): ReactNode => {
    if (entry.url) {
        return (
            <div
                className={cn("grid grid-cols-[1fr_auto] w-full px-3 pl-2 rounded-lg cursor-pointer", className)}
            >
                <div
                    className={cn("not-prose w-full text-sm", entry.url ? "cursor-pointer" : "cursor-default")} dangerouslySetInnerHTML={{ __html: entry.html }} onClick={(e) => {
                        e.preventDefault()
                        e.stopPropagation()
                        if (onClick) {
                            onClick()
                        }
                    }}
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
            onClick={(e) => {
                e.preventDefault()
                e.stopPropagation()
                if (onClick) {
                    onClick()
                }
            }}
        />
    )
}


BibliographyEntryComponent.displayName = "BibliographyEntryComponent"
