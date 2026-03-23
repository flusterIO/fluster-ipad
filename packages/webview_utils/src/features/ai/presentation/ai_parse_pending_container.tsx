import React, { type ReactNode } from 'react'
import { FlusterAiParseButton } from "./ai_parse_button"
import { InlineMdxContent } from '#/mdx/components/inline_mdx_content'
import { type ParsedCodeBlock } from "../../../core/code_gen/typeshare/conundrum"
import { connect } from 'react-redux';
import { type GlobalWebviewStateDeepNullable } from '#/webview_global_state/cross_language_state_types';
import consola from 'consola';
import { type AiState, AiStateEvents, FoundationModelAccessStatus, type GeneralAiRequestPhase2Event } from '@/code_gen/typeshare/fluster_core_utilities';
import { type AiErrorState } from './types';
import { AiParsePendingErrorStatusContainer } from './ai_parse_pending_error_status_container';
import { sendToSwift } from '@/utils/bridge/send_to_swift';


interface FlusterAiParsePendingContainerProps {
    /**
     * stringifiedResult is the stringified JSON of the CodeBlockParsingResult
     * that created the component so the data can be sent to swift when 'generate' is clicked 
     * with the current state of the component, not necessarily the current state of the note
     * to reflect the state that the user is actually seeing as the DB lags behind a bit.
     */
    res: ParsedCodeBlock | null
    status: AiState["foundation_model_access"]
}

const connector = connect((state: GlobalWebviewStateDeepNullable) => ({
    status: state.ai.foundation_model_access
}))


export const FlusterAiParsePendingContainer = connector(({ res, status }: FlusterAiParsePendingContainerProps): ReactNode => {
    if (!res) {
        consola.error("Something went wrong while attempting to parse the json data passed into the FlusterAiParsePendingContainer.")
        return null
    }

    if (status === FoundationModelAccessStatus.Available) {
        return (
            <div
                className="w-full max-w-[min(1080px,90%)] mx-auto p-4 border border-fd-ring bg-fd-card rounded text-card-foreground [&_*]:text-card-foreground @container/aiParsePendingContainer relative shadow-primary/50 border hover:border-primary transition-shadow duration-500"
            >
                <div
                    className="absolute inset-x-0 top-0 h-px rounded-t-xl bg-gradient-to-r from-transparent via-primary/50 to-transparent"
                    aria-hidden="true"
                />
                <div className="flex flex-col justify-start items-start">
                    <div className="text-lg font-bold">AI Prompt:</div>
                    <InlineMdxContent
                        mdx={res.content}
                        className="indent-4"
                    />
                </div>
                <div className="w-full flex flex-row justify-end items-center">
                    <div className="invisible @[540px]/aiParsePendingContainer:visible absolute bottom-3 left-3 text-sm opacity-60">
                        {`See 'AI?' docs for supported actions`}
                    </div>
                    <FlusterAiParseButton
                        onClick={() => {
                            sendToSwift(AiStateEvents.SendGeneralAiRequestPhase2, {
                                full_match: res.full_match,
                                user_request: res.content
                            } satisfies GeneralAiRequestPhase2Event)
                        }}
                    />
                </div>
            </div>
        )
    }


    /* eslint-disable-next-line  -- You fucking better not reformat this... */
    const errors: { [K in Exclude<FoundationModelAccessStatus, "available">]: AiErrorState } = {
        [FoundationModelAccessStatus.ModelNotReady]: {
            message: () => {
                return (
                    <>
                        Apple is telling us that your model is not ready.This usually means that it is still downloading, but if this problem continues, please file an issue on < a href="https://github.com/flusterIO" > Github</a >
                    </>
                )
            }
        },
        [FoundationModelAccessStatus.AppleIntelligenceNotEnabled]: {
            message: () => {
                return (
                    <>
                        You must enable Apple Intelligence to use this feature. Behind the scenes this downloads the necessary models to your device that Fluster uses for all of it's _local_ AI inference.
                    </>
                )
            }
        },
        [FoundationModelAccessStatus.DeviceNotEligible]: {
            message: () => {
                return (
                    <>
                        Unfortunately it looks like this device is not eligable for Apple Intelligence. This feature is required for all local AI inside of Fluster, but a future version will incorporate server scale AI.
                    </>
                )
            }
        },
        [FoundationModelAccessStatus.UnknownStatus]: {
            message: () => {
                return (
                    <>
                        Something went wrong while attempting to check the availability of the models Fluster needs for it's AI inference. If this continues, please file an issue on <a href="https://github.com/flusterIO">Github</a>
                    </>
                )
            }
        }
    }


    return (
        <AiParsePendingErrorStatusContainer
            error={errors[status]}
        />
    )


})


FlusterAiParsePendingContainer.displayName = "FlusterAiParsePendingContainer"
