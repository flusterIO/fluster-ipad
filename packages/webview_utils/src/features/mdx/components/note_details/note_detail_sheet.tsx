import { NoteDetailWebviewActions, NoteDetailWebviewEvents } from '@/code_gen/typeshare/fluster_core_utilities';
import { H1, H3, H4 } from '@/shared_components/typography/typography';
import { useEventListener } from '@/state/hooks/use_event_listener';
import { sendToSwift } from '@/utils/bridge/send_to_swift';
import React, { useEffect, useMemo, useState, type ReactNode } from 'react'
import { ByteBuffer } from 'flatbuffers';
import { MdxSerialization } from '@/code_gen/flat_buffer/v1_flat_buffer_schema';
import { NoteDetailDataBuffer } from '@/code_gen/flat_buffer/mdx-serialization/note-details';
import { LoadingComponent } from '@/shared_components/loading_component';
import { InlineMdxContent } from '../inline_mdx_content';
import { CitationResultBuffer, TagResultBuffer } from '@/code_gen/flat_buffer/mdx-serialization';


declare global {
    interface WindowEventMap {
        [NoteDetailWebviewEvents.SetNoteDetails]: CustomEvent<number[]>;
    }
}


const Subtitle = ({ children }: { children: ReactNode }): ReactNode => {
    return (
        <H3 className="w-full text-muted-foreground tracking-wide">{children}</H3>
    )
}

export const NoteDetailSheet = (): ReactNode => {
    const [data, setData] = useState<NoteDetailDataBuffer | null>(null)
    useEventListener(NoteDetailWebviewEvents.SetNoteDetails, (e) => {
        console.log("e: ", e)
        try {
            const bytes = new Uint8Array(e.detail);
            const buf = new ByteBuffer(bytes)
            const noteDetails = MdxSerialization.NoteDetails.NoteDetailDataBuffer.getRootAsNoteDetailDataBuffer(buf);
            setData(noteDetails)
        } catch (err) {
            console.log("NoteDetails serialization error: ", err)
        }
    })

    useEffect(() => {
        if (!data) {
            sendToSwift(NoteDetailWebviewActions.RequestNoteDetailData)
        } else {
            sendToSwift(NoteDetailWebviewActions.SetWebviewLoaded)
            document.body.classList.remove("loading")
        }
    }, [data])

    const tags = useMemo(() => {
        if (!data) {
            return null
        }
        const t: TagResultBuffer[] = []
        for (let i = 0; i < data.tagsLength(); i++) {
            const x = data.tags(i)
            if (x) {
                t.push(x)
            }
        }
        return t
    }, [data])
    const citations = useMemo(() => {
        if (!data) {
            return null
        }
        const t: CitationResultBuffer[] = []
        for (let i = 0; i < data.citationsLength(); i++) {
            const x = data.citations(i)
            if (x) {
                t.push(x)
            }
        }
        return t
    }, [data])

    if (!data) {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center p-8">
                <LoadingComponent />
            </div>
        )
    }

    const summary = data.summary()


    return (
        <div className="w-full h-full flex flex-col justify-start items-center px-8 py-12">
            <div className="w-full h-screen loading-hide max-w-[768px]">
                <Subtitle>Title</Subtitle>
                <div className="block scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
                    <InlineMdxContent className="pb-4 pt-2" mdx={`# ${data.title() ?? "No title found"}`} />
                </div>
                <div className="text-muted-foreground text-light">{`Last modified ${data.lastModifiedString() ?? 'unknown'}`}</div>
                <div className="text-muted-foreground text-light">{`Last read ${data.lastReadString() ?? 'unknown'}`}</div>
                <div className="w-full h-[2px] bg-muted-foreground/60 my-6" />
                {summary ? (
                    <>
                        <Subtitle>Summary</Subtitle>
                        <div className="text-lg text-foreground/80">{summary}</div>
                    </>
                ) : null}
                <Subtitle>{`Tags (${tags?.length ?? 0})`}</Subtitle>
                {tags?.length ? (
                    <div className="flex flex-row justify-start items-center gap-4">
                        {tags.map((t) => (
                            <div className="bg-primary/70 text-primary-foreground rounded-lg px-2 py-1">
                                {t.body()}
                            </div>
                        ))}
                    </div>
                ) : (
                    <div className="w-full flex flex-row justify-center items-center">
                        <div className="text-lg text-muted-foreground">
                            No tags
                        </div>
                    </div>
                )}
                <Subtitle>{`Citations (${data.citationsLength()})`}</Subtitle>
                {citations?.length ? citations.map((c) => {
                    return (
                        <div className="w-full px-4 py-3 rounded-lg">
                            <div className="font-bold text-lg">
                                <InlineMdxContent abortIfNoMath mdx={c.body() ?? ""} />
                            </div>
                        </div>
                    )
                }) : (
                    <div className="flex flex-row justify-center items-center px-4">
                        <div className="text-lg text-muted-foreground">No citations</div>
                    </div>
                )}
            </div>
        </div>
    )
}


NoteDetailSheet.displayName = "NoteDetailSheet"
