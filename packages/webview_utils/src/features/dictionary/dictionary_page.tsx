import React, { type ReactNode } from 'react'
import { DictionaryEntryComponent } from './hooks/dictionary_entry'
import { type DictionaryState, DictionaryWebviewIds } from '@/code_gen/typeshare/fluster_core_utilities'
import { H1 } from '@/shared_components/typography/typography'
import { InlineMdxContent } from '#/mdx/components/inline_mdx_content'
import { MdxContent } from '#/mdx/components/mdx_content'
import { connect } from 'react-redux';
import { type GlobalAppState } from '#/webview_global_state/store';

const connector = connect((state: GlobalAppState) => ({
    entries: state.dictionary.entries
}))

export const DictionaryPage = connector(({ entries }: Pick<DictionaryState, "entries">): ReactNode => {
    return (
        <div
            className="w-full h-fit min-h-screen"
            id={DictionaryWebviewIds.DictionaryContainer}
        >
            <div className="w-full max-w-[1080px] px-8 ml-auto mr-auto">
                <H1 className="mb-8 hide-desktop">Dictionary</H1>
                {entries.map((entry) => {
                    return (
                        <DictionaryEntryComponent
                            InlineMdxContent={InlineMdxContent}
                            entryData={{
                                note_id: entry.origin_note_id,
                                label: entry.label
                            }}
                        >
                            <MdxContent
                                mdx={entry.body}
                            />
                        </DictionaryEntryComponent>
                    )
                })}
            </div>
        </div>
    )
})


DictionaryPage.displayName = "DictionaryPage"
