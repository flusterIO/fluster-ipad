import { NoteDetailWebviewActions, NoteDetailWebviewEvents } from '@/code_gen/typeshare/fluster_core_utilities';
import { H3 } from '@/shared_components/typography/typography';
import { sendToSwift } from '@/utils/bridge/send_to_swift';
import React, { useEffect, useMemo, type ReactNode } from 'react'
import { NoteDetailCitationBuffer, NoteDetailDataBuffer } from '@/code_gen/flat_buffer/mdx-serialization/note-details';
import { LoadingComponent } from '@/shared_components/loading_component';
import { InlineMdxContent } from '../inline_mdx_content';
import { TagResultBuffer } from '@/code_gen/flat_buffer/mdx-serialization';
import { setWindowBridgeFunctions } from '#/editor/code_editor/types/swift_events/swift_events';
import { setWebviewWindowBridgeFunctions } from '#/webview_container/state/swift_events/webview_swift_events';
import { ErrorBoundary } from 'react-error-boundary';
import { TaggableBadge } from '@/shared_components/shad/badge';
import { NoteDetailsBibliography } from './note_details_bibliography';
import { Subtitle } from './subtitle';

setWindowBridgeFunctions();
setWebviewWindowBridgeFunctions();

declare global {
    interface WindowEventMap {
        [NoteDetailWebviewEvents.SetNoteDetails]: CustomEvent<number[]>;
    }
}

export const NoteDetailSheet = ({ data }: { data: NoteDetailDataBuffer }): ReactNode => {
    useEffect(() => {
        sendToSwift(NoteDetailWebviewActions.SetWebviewLoaded);
        document.body.classList.remove("loading");
    }, [])

    const tags = useMemo(() => {
        if (!data) {
            return null
        }
        const t: TagResultBuffer[] = []
        for (let i = 0; i < data.tagsLength(); i++) {
            const x = data.tags(i)
            if (x) {
                t.push(x)
            } else {
                console.error("Could not load tag")
            }
        }
        return t
    }, [data]);

    if (!data) {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center p-8">
                <LoadingComponent />
            </div>
        )
    }

    const summary = data.summary()
    const topic = data.topic()
    const subject = data.subject()


    return (
        <ErrorBoundary
            fallbackRender={() => (
                <div className="w-full h-full flex flex-col justify-center items-center px-6">
                    <div className="text-xl text-foreground/80">An error occurred while gathering your note's details</div>
                </div>
            )}
        >
            <div className="w-full h-full flex flex-col justify-start items-center px-8 py-12">
                <div className="w-full h-screen loading-hide max-w-[768px]">
                    <Subtitle>Title</Subtitle>
                    <div className="block scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
                        <InlineMdxContent className="pb-4 pt-2" mdx={`# ${data.title() ?? "No title found"}`} />
                    </div>
                    <div className="text-muted-foreground text-light">{`Last modified ${data.lastModifiedString() ?? 'unknown'}`}</div>
                    <div className="w-full h-[2px] bg-muted-foreground/60 my-6" />
                    {summary ? (
                        <div className="flex flex-row justify-start items-center gap-x-6">
                            <Subtitle>Summary</Subtitle>
                            <div className="text-lg text-foreground/80">{summary}</div>
                        </div>
                    ) : null}
                    {topic ? (
                        <div className="flex flex-row justify-start items-center gap-x-6 my-6">
                            <Subtitle className="w-fit">Topic</Subtitle>
                            <TaggableBadge
                                taggableValue={topic}
                                clickAction={NoteDetailWebviewActions.HandleTopicClick}
                            />
                        </div>
                    ) : null}
                    {subject ? (
                        <div className="flex flex-row justify-start items-center gap-x-6 my-6">
                            <Subtitle className="w-fit">Subject</Subtitle>
                            <TaggableBadge
                                taggableValue={subject}
                                clickAction={NoteDetailWebviewActions.HandleSubjectClick}
                            />
                        </div>
                    ) : null}
                    <Subtitle>{`Tags (${tags?.length ?? 0})`}</Subtitle>
                    {tags?.length ? (
                        <div className="flex flex-row justify-start items-center gap-4 mt-2 mb-4">
                            {tags.map((t) => (
                                <button
                                    className="bg-primary/70 text-primary-foreground rounded-lg px-2 py-1 cursor-pointer"
                                    onClick={() => {
                                        const b = t.body()
                                        if (b !== null) {
                                            sendToSwift(NoteDetailWebviewActions.HandleTagClick, b)
                                        } else {
                                            console.error("No tag body found")
                                        }
                                    }}
                                >
                                    {t.body()}
                                </button>
                            ))}
                        </div>
                    ) : (
                        <div className="w-full flex flex-row justify-center items-center mt-2 mb-4">
                            <div className="text-lg text-muted-foreground">
                                No tags
                            </div>
                        </div>
                    )}
                    <NoteDetailsBibliography
                        bibliographyContent={Array(data.citationsLength()).fill(0).map((_, i) => data.citations(i) as NoteDetailCitationBuffer)}
                    />
                </div>
            </div>
        </ErrorBoundary>
    )
}


NoteDetailSheet.displayName = "NoteDetailSheet"
