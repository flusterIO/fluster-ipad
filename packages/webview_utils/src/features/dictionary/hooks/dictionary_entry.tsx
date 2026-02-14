import { EditorClient } from '#/editor/code_editor/data/editor_client';
import { shrinkMdxClasses } from '#/mdx/components/inline_mdx_classes';
import type { MdxContentProps } from '#/mdx/components/mdx_content';
import { cn } from '@/utils/cn';
import React, { FC, type ReactNode } from 'react'


interface DictionaryEntryComponentProps {
    /* entry: DictionaryEntryModel | null */
    label: string;
    children: string;
    /** noteId matches the id field on the NoteModel, note the user provided id. */
    noteId?: string;
    InlineMdxContent: FC<MdxContentProps>;
}

export const DictionaryEntryComponent = ({ children, label, noteId, InlineMdxContent }: DictionaryEntryComponentProps): ReactNode => {
    return (
        <div className="w-full my-6 max-w-[1080px]">
            <InlineMdxContent
                className={cn("[&_p]:text-xl font-serif font-medium tracking-tight text-foreground decoration-none underline-none", noteId ? "cursor-pointer" : "cursor-default")}
                role="button"
                onClick={() => {
                    if (noteId) {
                        EditorClient.navigateToNoteById(noteId)
                    }
                }}
                mdx={label}
            />
            <div className={cn("w-full ml-4", shrinkMdxClasses)}>
                {children}
            </div>
        </div>
    )
}


DictionaryEntryComponent.displayName = "DictionaryEntryComponent"
