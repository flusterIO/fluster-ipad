import { type NoteDetailState, NoteDetailWebviewActions, type NoteDetailWebviewEvents } from '@/code_gen/typeshare/fluster_core_utilities';
import { sendToSwift } from '@/utils/bridge/send_to_swift';
import React, { useEffect, type ReactNode } from 'react'
import { LoadingComponent } from '@/shared_components/loading_component';
import { ErrorBoundary } from 'react-error-boundary';
import { TaggableBadge } from '@/shared_components/shad/badge';
import { NoteDetailsBibliography } from './note_details_bibliography';
import { Subtitle } from './subtitle';

declare global {
    interface WindowEventMap {
        [NoteDetailWebviewEvents.SetNoteDetails]: CustomEvent<number[]>;
    }
}

import { connect } from 'react-redux';
import { type GlobalAppState } from '#/webview_global_state/store';
import { type WithNullableOptionals } from '../../../../core/utils/types/utility_types';
import { H1 } from '../../../../core/shared_components/typography/typography';
const connector = connect((state: GlobalAppState) => ({
    data: state.note_details
}))

export const NoteDetailSheet = connector(({ data }: { data: WithNullableOptionals<NoteDetailState> | null }): ReactNode => {
    useEffect(() => {
        if (!data) {
            sendToSwift(NoteDetailWebviewActions.RequestNoteDetailData)
        } else {
            sendToSwift(NoteDetailWebviewActions.SetWebviewLoaded);
            document.body.classList.remove("loading");
        }
    }, [data])

    if (!data) {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center p-8">
                <LoadingComponent />
            </div>
        )
    }


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
                    <H1 className="text-4xl font-extrabold tracking-tight lg:text-5xl mb-3">{data.title}</H1>
                    <div className="text-muted-foreground text-light">{`Last modified ${data.last_modified_string}`}</div>
                    <div className="w-full h-[2px] bg-muted-foreground/60 mb-6 mt-3" />
                    {data.summary ? (
                        <div className="flex flex-row justify-start items-center gap-x-6">
                            <Subtitle>Summary</Subtitle>
                            <div className="text-lg text-foreground/80">{data.summary}</div>
                        </div>
                    ) : null}
                    {data.topic ? (
                        <div className="flex flex-row justify-start items-center gap-x-3 mt-6 mb-0">
                            <Subtitle className="w-fit h-fit">Topic</Subtitle>
                            <TaggableBadge
                                taggableValue={data.topic}
                                className="cursor-pointer"
                                clickAction={NoteDetailWebviewActions.OnTopicClick}
                            />
                        </div>
                    ) : null}
                    {data.subject ? (
                        <div className="flex flex-row justify-start items-center gap-x-3 mb-6">
                            <Subtitle className="w-fit h-fit">Subject</Subtitle>
                            <TaggableBadge
                                taggableValue={data.subject}
                                className="cursor-pointer"
                                clickAction={NoteDetailWebviewActions.OnSubjectClick}
                            />
                        </div>
                    ) : null}
                    <Subtitle>{`Tags (${data.tags.length})`}</Subtitle>
                    {data.tags.length ? (
                        <div className="flex flex-row justify-start items-center gap-4 mt-2 mb-4">
                            {data.tags.map((t) => (
                                <button
                                    className="bg-primary/70 text-primary-foreground rounded-lg px-2 py-1 cursor-pointer"
                                    onClick={() => {
                                        sendToSwift(NoteDetailWebviewActions.OnTagClick, t.body)
                                    }}
                                >
                                    {t.body}
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
                    />
                </div>
            </div>
        </ErrorBoundary>
    )
})


NoteDetailSheet.displayName = "NoteDetailSheet"
