import { EditorClient } from '#/editor/code_editor/data/editor_client';
import { shrinkMdxClasses } from '#/mdx/components/inline_mdx_classes';
import { type MdxContentProps } from '#/mdx/components/mdx_content_types';
import { type DictionaryEntryResultData } from '@/code_gen/typeshare/conundrum';
import { cn } from '@/utils/cn';
import React, { type FC, type ReactNode } from 'react'


interface DictionaryEntryComponentProps {
    children: ReactNode;
    InlineMdxContent: FC<MdxContentProps>;
    /** note_id matches the id field on the NoteModel, note the user provided id. */
    entryData: DictionaryEntryResultData
}

export const DictionaryEntryComponent = ({ children, entryData, InlineMdxContent }: DictionaryEntryComponentProps): ReactNode => {
    const { label, note_id } = entryData
    return (
        <div className="w-full my-6 max-w-[1080px] font-serif">
            <InlineMdxContent
                className={cn("[&_p]:text-xl font-medium tracking-tight text-foreground decoration-none underline-none", note_id ? "cursor-pointer" : "cursor-default")}
                role="button"
                onClick={() => {
                    if (note_id) {
                        EditorClient.navigateToNoteById(note_id)
                    }
                }}
                mdx={label}
            />
            <div className={cn("w-full pl-4", shrinkMdxClasses)}>
                {children}
            </div>
        </div>
    )
}


DictionaryEntryComponent.displayName = "DictionaryEntryComponent"
