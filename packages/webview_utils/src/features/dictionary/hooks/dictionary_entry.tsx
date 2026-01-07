/* import { MdxContent } from '#/mdx/components/mdx_content' */
import React, { type ReactNode } from 'react'


interface DictionaryEntryComponentProps {
    /* entry: DictionaryEntryModel | null */
    label: string;
    body: string;
    /** noteId matches the id field on the NoteModel, note the user provided id. */
    noteId?: string;
}

export const DictionaryEntryComponent = ({ label }: DictionaryEntryComponentProps): ReactNode => {
    return (
        <div className="w-full mt-8">
            <h2 className="text-3xl font-serif font-medium tracking-tight text-foreground">{label}</h2>
            {/* <MdxContent */}
            {/*     mdx={body} */}
            {/*     className="mt-4 pl-8" */}
            {/* /> */}
        </div>
    )
}


DictionaryEntryComponent.displayName = "DictionaryEntryComponent"
