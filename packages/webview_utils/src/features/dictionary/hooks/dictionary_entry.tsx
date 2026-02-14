import { EditorClient } from '#/editor/code_editor/data/editor_client';
/* import { MdxContent } from '#/mdx/components/mdx_content'; */
/* import { InlineMdxContent } from '#/mdx/components/inline_mdx_content'; */
/* import { MdxContent } from '#/mdx/components/mdx_content'; */
import { cn } from '@/utils/cn';
import React, { FC, type ReactNode } from 'react'


interface DictionaryEntryComponentProps {
    /* entry: DictionaryEntryModel | null */
    label: string;
    children: string;
    /** noteId matches the id field on the NoteModel, note the user provided id. */
    noteId?: string;
    InlineMdxContent: FC<{ mdx: string }>;
}

export const DictionaryEntryComponent = ({ children, label, noteId, InlineMdxContent }: DictionaryEntryComponentProps): ReactNode => {
    return (
        <div className="w-full my-6 max-w-[1080px] bg-card border rounded p-4">
            <a
                className={cn("text-xl font-serif font-medium tracking-tight text-foreground", noteId ? "cursor-pointer" : "cursor-default")}
                role="button"
                onClick={() => {
                    if (noteId) {
                        EditorClient.navigateToNoteById(noteId)
                    }
                }}
            >
                <InlineMdxContent
                    mdx={label}
                />
            </a>
            <div className="w-full">
                {children}
            </div>
        </div>
    )
}


DictionaryEntryComponent.displayName = "DictionaryEntryComponent"
