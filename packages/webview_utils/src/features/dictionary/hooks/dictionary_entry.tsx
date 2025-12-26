import { MdxContent } from '#/mdx/components/mdx_content'
import { DictionaryEntryModel } from '@/code_gen/flat_buffer/dictionary'
import React, { type ReactNode } from 'react'


interface DictionaryEntryComponentProps {
    entry: DictionaryEntryModel | null
}

export const DictionaryEntryComponent = ({ entry }: DictionaryEntryComponentProps): ReactNode => {
    if (entry === null) {
        return null
    }
    return (
        <div className="w-full mt-8">
            <h2 className="text-3xl font-serif font-medium tracking-tight text-foreground">{entry.label() ?? ""}</h2>
            <MdxContent
                mdx={entry.body() ?? ""}
                className="mt-4 pl-8"
            />
        </div>
    )
}


DictionaryEntryComponent.displayName = "DictionaryEntryComponent"
