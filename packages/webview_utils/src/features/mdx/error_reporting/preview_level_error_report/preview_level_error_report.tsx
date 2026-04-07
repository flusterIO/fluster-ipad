import { type MdxContentProps } from '#/mdx/components/mdx_content_types'
import { useDebounceMdxParse } from '#/mdx/hooks/use_debounce_mdx_parse'
import { TriangleAlert } from 'lucide-react'
import React, { useEffect, type ReactNode } from 'react'
import { type FallbackProps } from 'react-error-boundary'
import { motion } from "framer-motion"
import { useEventListener } from '@/state/hooks/use_event_listener'
import { SplitviewEditorWebviewEvents } from '@/code_gen/typeshare/fluster_core_utilities'
import { useDispatch } from 'react-redux'
import { type ConundrumError } from '@/code_gen/typeshare/conundrum'
import { connect } from 'react-redux';
import { type GlobalWebviewStateDeepNullable } from '#/webview_global_state/cross_language_state_types'
import { clearConundrumErrors } from '#/webview_global_state/conundrum_state/conundrum_slice'
import { ConundrumErrorsReport } from './conundrum_error_report'


declare global {
    interface WindowEventMap {
        [SplitviewEditorWebviewEvents.ErrorStateReset]: CustomEvent<null>
    }
}


const connector = connect((state: GlobalWebviewStateDeepNullable) => ({
    conundrumErrors: state.conundrum.errors
}))


export const PreviewLevelErrorReport = connector(({ debounceTimeout, showWebviewAction, additionalComponents, id, mdx, resetErrorBoundary, conundrumErrors }: Partial<FallbackProps> & Pick<MdxContentProps, "debounceTimeout" | "showWebviewAction" | "additionalComponents"> & { id: string, mdx: string, conundrumErrors?: ConundrumError[] }): ReactNode => {
    // This is required to eventually 'unlock' the error report once parsing succeeds.
    const { setValue, value } = useDebounceMdxParse(
        undefined,
        debounceTimeout,
        id,
        showWebviewAction,
        additionalComponents
    );
    useEffect(() => {
        if (mdx !== value) {
            setValue(mdx)
        }
    }, [mdx, setValue, value])

    const dispatch = useDispatch();

    useEventListener(SplitviewEditorWebviewEvents.ErrorStateReset, () => {
        if (resetErrorBoundary) {
            resetErrorBoundary();
        }
        dispatch(clearConundrumErrors())
    })


    const totalErrors = conundrumErrors?.length ?? 1;

    return (
        <div className="@container/preview-error w-full px-4 h-screen z-999 flex flex-col justify-center items-center absolute top-0 left-0 right-0 bottom-0 border rounded">
            <motion.div
                className="w-full max-w-[540px] flex flex-col justify-center items-center border pt-4 px-5h"
                initial={{
                    scale: 0,
                    opacity: 0
                }}
                animate={{
                    scale: 1,
                    opacity: 1
                }}
            >
                <div className="w-full flex @xs/preview-error:flex-row flex-col justify-start items-center gap-x-3 px-4">
                    <div className="rounded-lg bg-destructive/20 p-3">
                        <TriangleAlert className="stroke-destructive" />
                    </div>
                    <div className="flex flex-col justify-start items-start gap-y-0">
                        <div className="fold-bold my-0">Conundrum</div>
                        <div className="text-sm text-fd-card-foreground/70 my-0">{`Found ${totalErrors} error${totalErrors > 1 ? "s" : ""}`}</div>
                    </div>
                </div>
                <div className="w-full flex flex-col justify-center items-center @[540px]:inline-block pb-3">
                    {conundrumErrors?.length ? (
                        <ConundrumErrorsReport errors={conundrumErrors} />
                    ) : (
                        <>
                            <div className="font-bold mt-6 mb-4 px-4">Error</div>
                            <div className="text-muted-foreground prose-sm [&_p]:text-muted-foreground pb-4 px-4">
                                Fluster encountered an error while attempting to parse this document. Please double-check your syntax.
                            </div>
                        </>
                    )}
                </div>
            </motion.div>
        </div>
    )
})


PreviewLevelErrorReport.displayName = "PreviewLevelErrorReport"
