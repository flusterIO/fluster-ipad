import { FormattedCitationData, getFormattedCitations } from '#/bibliography/methods/parse_bib_entries'
import { NoteDetailCitationBuffer } from '@/code_gen/flat_buffer/mdx-serialization/note-details'
import React, { useEffect, useEffectEvent, useState, type ReactNode } from 'react'
import { Subtitle } from './subtitle'



interface NoteDetailsBibliographyProps {
    bibliographyContent: NoteDetailCitationBuffer[]
}

export const NoteDetailsBibliography = (props: NoteDetailsBibliographyProps): ReactNode => {
    const [data, setData] = useState<FormattedCitationData[]>([])
    const handleData = useEffectEvent(async (content: string) => {
        const res = await getFormattedCitations(content, props.bibliographyContent)
        setData(res.formattedItems)
    })
    useEffect(() => {
        let s = ""
        for (const item of props.bibliographyContent) {
            s += `${item.body()}\n\n`
        }
        handleData(s)
    }, [props.bibliographyContent])
    return (
        <>
            <Subtitle>{`Citations (${props.bibliographyContent.length})`}</Subtitle>
            {data?.length ? (
                <div className="w-full flex flex-col justify-start items-center gap-4 py-4">
                    {data.slice(0, 5).map((c) => {
                        return (
                            <div className="w-full px-4 py-3 rounded-lg bg-muted/30 border">
                                <div className="font-bold text-lg text-foreground" dangerouslySetInnerHTML={{ __html: c.html }} />
                            </div>
                        )
                    })}
                    {data.length > 1 ? (
                        <div>
                            {/* <Button></Button> */}
                        </div>
                    ) : null}
                </div>
            ) : (
                <div className="flex flex-row justify-center items-center px-4">
                    <div className="text-lg text-muted-foreground">No citations</div>
                </div>
            )}
        </>
    )
}


NoteDetailsBibliography.displayName = "NoteDetailsBibliography"
