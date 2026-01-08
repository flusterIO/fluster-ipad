import { FormattedCitationData, getFormattedCitations } from '#/bibliography/methods/parse_bib_entries'
import { NoteDetailCitationBuffer } from '@/code_gen/flat_buffer/mdx-serialization/note-details'
import React, { useEffect, useEffectEvent, useState, type ReactNode } from 'react'
import { Subtitle } from './subtitle'
import { Button } from '@/shared_components/shad/button'
import { ChevronLeft, ChevronRight } from 'lucide-react'
import { sendToSwift } from '@/utils/bridge/send_to_swift'
import { NoteDetailWebviewActions } from '@/code_gen/typeshare/fluster_core_utilities'

const PER_PAGE = 5


interface NoteDetailsBibliographyProps {
    bibliographyContent: NoteDetailCitationBuffer[]
}

export const NoteDetailsBibliography = (props: NoteDetailsBibliographyProps): ReactNode => {
    const [data, setData] = useState<FormattedCitationData[]>([])
    const [citationsPage, setCitationsPage] = useState(0)
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
    const maxCitationsPage = Math.floor(data.length / PER_PAGE)
    return (
        <>
            <Subtitle>{`Citations (${props.bibliographyContent.length})`}</Subtitle>
            {data?.length ? (
                <div className="w-full flex flex-col justify-start items-center gap-4 py-4">
                    {data.slice(citationsPage * PER_PAGE, PER_PAGE).map((c) => {
                        return (
                            <div key={c.id} className="w-full px-4 py-3 rounded-lg bg-muted/30 border cursor-pointer"
                                onClick={() => {
                                    // c.id is either the citationKey of the unique database uuid. The citationKey is used if one exists (it always should), and then 
                                    // the database uuid is used as a fallback.
                                    sendToSwift(NoteDetailWebviewActions.HandleCitationClick, c.id)
                                }}
                            >
                                <div className="font-bold text-lg text-foreground" dangerouslySetInnerHTML={{ __html: c.html }} />
                            </div>
                        )
                    })}
                    {data.length > PER_PAGE ? (
                        <div className="w-full h-fit flex flex-row justify-end items-center gap-4">
                            <Button
                                variant="outline"
                                size="icon"
                                aria-label="Back"
                                disabled={citationsPage === 0}
                                onClick={() => setCitationsPage(citationsPage - 1)}
                            >
                                <ChevronLeft className="text-foreground" />
                            </Button>
                            <Button
                                variant="outline"
                                size="icon"
                                aria-label="Next"
                                disabled={citationsPage === maxCitationsPage}
                                onClick={() => setCitationsPage(citationsPage + 1)}
                            >
                                <ChevronRight className="text-foreground" />
                            </Button>
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
