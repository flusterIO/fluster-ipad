import { NoteDetailWebviewActions, NoteDetailWebviewEvents } from '@/code_gen/typeshare/fluster_core_utilities';
import { H4 } from '@/shared_components/typography/typography';
import { useEventListener } from '@/state/hooks/use_event_listener';
import { sendToSwift, SwiftHandler } from '@/utils/bridge/send_to_swift';
import React, { useEffect, useState, type ReactNode } from 'react'

// TODO: Replace all of these local references with the flatbuffers crate once things are serializing on the rust side properly.
interface LocalFrontMatterResult {
    ignore_parsers: string[];
    title?: string;
    user_defined_id?: string;
}

interface LocalMdxParsingResult {
    content: string;
    tags: { body: string }[]
    front_matter?: LocalFrontMatterResult
    citations: {
        citation_key: string;
        body: string;
    }[]
}

declare global {

    interface WindowEventMap {
        "set-note-details": CustomEvent<string>;
    }
}

const Subtitle = ({ children }: { children: ReactNode }): ReactNode => {
    return (
        <H4>{children}</H4>
    )
}

export const NoteDetailSheet = (): ReactNode => {
    const [data, setData] = useState<LocalMdxParsingResult | null>(null)
    useEventListener(NoteDetailWebviewEvents.SetNoteDetails, (e) => {
        try {
            const r: LocalMdxParsingResult = JSON.parse(e.detail);
            setData(r)
        } catch (err) {
            console.log("Parsing error: ", err);
        }
    })

    useEffect(() => {
        if (!data) {
            sendToSwift(NoteDetailWebviewActions.RequestNoteDetailData)
        }
    }, [])

    return (
        <div className="w-full h-screen flex flex-col justify-center items-center">
            <Subtitle>Subject</Subtitle>
            {`${data ? "Has data" : "No data"}`}
        </div>
    )
}


NoteDetailSheet.displayName = "NoteDetailSheet"
