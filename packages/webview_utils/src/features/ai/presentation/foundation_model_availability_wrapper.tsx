import React, { type ReactNode } from 'react'
import { connect } from 'react-redux';
import { type GlobalWebviewStateDeepNullable } from '#/webview_global_state/cross_language_state_types';
import { type AiState, FoundationModelAccessStatus } from '@/code_gen/typeshare/fluster_core_utilities';
import { type AiErrorState } from './types';
import { AiParsePendingErrorStatusContainer } from './ai_parse_pending_error_status_container';


interface FlusterAiParsePendingContainerProps {
    /* res: ParsedCodeBlock | null */
    status: AiState["foundation_model_access"]
    /// A simple development utilty to force the primary continer to be shown.
    adfcbzadfjadfadfkhllakadf?: boolean
    children: ReactNode
}

const connector = connect((state: GlobalWebviewStateDeepNullable) => ({
    status: state.ai.foundation_model_access
}))


export const FoundationModelAvailabilityWrapper = connector(({ status, children, adfcbzadfjadfadfkhllakadf }: FlusterAiParsePendingContainerProps): ReactNode => {

    if (status === FoundationModelAccessStatus.Available || adfcbzadfjadfadfkhllakadf) {
        return children
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
                        You must enable Apple Intelligence to use this feature. Behind the scenes this downloads the necessary models to your device that Fluster uses for all of it's <span className="italic">local</span> AI inference.
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


FoundationModelAvailabilityWrapper.displayName = "FoundationModelAvailabilityWrapper"
