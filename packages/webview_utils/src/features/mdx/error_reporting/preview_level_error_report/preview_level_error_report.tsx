import { type MdxContentProps } from '#/mdx/components/mdx_content_types'
import { useDebounceMdxParse } from '#/mdx/hooks/use_debounce_mdx_parse'
import { TriangleAlert } from 'lucide-react'
import React, { useEffect, type ReactNode } from 'react'
import { type FallbackProps } from 'react-error-boundary'
import { motion } from "framer-motion"
import { useEventListener } from '@/state/hooks/use_event_listener'
import { SplitviewEditorWebviewActions, SplitviewEditorWebviewEvents } from '@/code_gen/typeshare/fluster_core_utilities'


interface EventProps {

}
declare global {

    interface WindowEventMap {
        [SplitviewEditorWebviewEvents.ErrorStateReset]: CustomEvent<null>
    }
}


export const PreviewLevelErrorReport = ({ debounceTimeout, showWebviewAction, additionalComponents, id, mdx, resetErrorBoundary }: FallbackProps & Pick<MdxContentProps, "debounceTimeout" | "showWebviewAction" | "additionalComponents"> & { id: string, mdx: string }): ReactNode => {
    // This is required to eventually 'unlock' the error report once parsing succeeds.
    const { setValue, value, isReady } = useDebounceMdxParse(
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

    useEffect(() => {
        /* if (isReady) { */
        /*     resetErrorBoundary() */
        /* } */
    }, [isReady, resetErrorBoundary])

    useEventListener(SplitviewEditorWebviewEvents.ErrorStateReset, () => { resetErrorBoundary(); })


    return (
        <div className="w-full h-screen z-999 flex flex-col justify-center items-center absolute top-0 left-0 right-0 bottom-0 border rounded">
            <motion.div
                className="w-full max-w-[540px] flex flex-col justify-center items-center gap-y-6 border py-4 px-5h"
                initial={{
                    scale: 0,
                    opacity: 0
                }}
                animate={{
                    scale: 1,
                    opacity: 1
                }}
            >
                <div className="rounded-lg bg-destructive/20 p-3">
                    <TriangleAlert className="stroke-destructive" />
                </div>
                <div className="w-full flex flex-col justify-center items-center @[540px]:inline-block">
                    <div className="font-bold">Error</div>
                    <div className="text-muted-foreground text-sm text-center">
                        Fluster encountered an error while attempting to parse this document. Please double-check your syntax.
                    </div>
                </div>
            </motion.div>
        </div>
    )
}


PreviewLevelErrorReport.displayName = "PreviewLevelErrorReport"
