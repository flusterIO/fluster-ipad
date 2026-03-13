import React, { useState, type ReactNode } from 'react'
import { Subtitle } from './subtitle'
import { Button } from '@/shared_components/shad/button'
import { ChevronLeft, ChevronRight } from 'lucide-react'
import { sendToSwift } from '@/utils/bridge/send_to_swift'
import { MdxPreviewWebviewActions, type NoteDetailState } from '@/code_gen/typeshare/fluster_core_utilities'
import { connect } from "react-redux";
import { type GlobalAppState } from '#/webview_global_state/store'
import { BibliographyEntryComponent } from '#/bibliography/presentation/bibliography_entry'

const PER_PAGE = 5


const connector = connect((state: GlobalAppState) => ({
    citations: state.note_details?.citations
}))

export const NoteDetailsBibliography = connector(({ citations = [] }: { citations?: NoteDetailState["citations"] }): ReactNode => {
    const [citationsPage, setCitationsPage] = useState(0)
    const maxCitationsPage = Math.floor(citations.length / PER_PAGE)
    return (
        <>
            <Subtitle>{`Citations (${citations.length})`}</Subtitle>
            {citations.length ? (
                <div className="w-full flex flex-col justify-start items-center gap-4 py-4">
                    {citations.slice(citationsPage * PER_PAGE, (citationsPage + 1) * PER_PAGE).map((c) => {
                        return (
                            <BibliographyEntryComponent
                                key={c.citation_key}
                                entry={c}
                                /* className='' */
                                onClick={() => {
                                    sendToSwift(MdxPreviewWebviewActions.OnCitationClick, c.citation_key)
                                }}
                            /* idx={{ */
                            /*     index: i, */
                            /*      */
                            /* }} */
                            />
                        )
                    })}
                    {citations.length > PER_PAGE ? (
                        <div className="w-full h-fit flex flex-row justify-end items-center gap-4">
                            <Button
                                variant="outline"
                                size="icon"
                                aria-label="Back"
                                disabled={citationsPage === 0}
                                onClick={() => {
                                    setCitationsPage(citationsPage - 1)
                                }}
                            >
                                <ChevronLeft className="text-foreground" />
                            </Button>
                            <Button
                                variant="outline"
                                size="icon"
                                aria-label="Next"
                                disabled={citationsPage === maxCitationsPage}
                                onClick={() => { setCitationsPage(citationsPage + 1); }}
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
})


NoteDetailsBibliography.displayName = "NoteDetailsBibliography"
