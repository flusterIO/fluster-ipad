import { DictionaryData } from '@/code_gen/flat_buffer/dictionary'
import React, { type ReactNode } from 'react'
import { DictionaryEntryComponent } from './hooks/dictionary_entry'
import { DictionaryWebviewIds } from '@/code_gen/typeshare/fluster_core_utilities'
import { H1 } from '@/shared_components/typography/typography'


interface DictionaryPageProps {
    data: DictionaryData
}

export const DictionaryPage = ({ data }: DictionaryPageProps): ReactNode => {
    return (
        <div
            className="w-full h-fit min-h-screen"
            id={DictionaryWebviewIds.DictionaryContainer}
        >
            <div className="w-full max-w-[1080px] px-8 ml-auto mr-auto">
                <H1 className="mb-8">Dictionary</H1>
                {Array(data.entriesLength()).fill(0).map((_, i) => {
                    const entry = data.entries(i);
                    return (
                        <DictionaryEntryComponent
                            label={entry?.label() ?? ""}
                        >
                            {entry?.body() ?? ""}
                        </DictionaryEntryComponent>
                    )
                })}
            </div>
        </div>
    )
}


DictionaryPage.displayName = "DictionaryPage"
