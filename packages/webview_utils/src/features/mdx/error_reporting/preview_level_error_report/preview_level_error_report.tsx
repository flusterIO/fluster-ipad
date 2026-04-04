import { type MdxContentProps } from '#/mdx/components/mdx_content_types'
import { useDebounceMdxParse } from '#/mdx/hooks/use_debounce_mdx_parse'
import { TriangleAlert } from 'lucide-react'
import React, { useEffect, type ReactNode } from 'react'
import { type FallbackProps } from 'react-error-boundary'
import { motion } from "framer-motion"
import { useEventListener } from '@/state/hooks/use_event_listener'
import { SplitviewEditorWebviewEvents } from '@/code_gen/typeshare/fluster_core_utilities'
import { useDispatch } from 'react-redux'
import { clearConundrumError } from '#/webview_global_state/conundrum_state/conundrum_slice'
import { type ConundrumError, type ConundrumErrorVariant } from '@/code_gen/typeshare/conundrum'
import { InlineMdxContent } from '#/mdx/components/inline_mdx_content'


declare global {
    interface WindowEventMap {
        [SplitviewEditorWebviewEvents.ErrorStateReset]: CustomEvent<null>
    }
}


const getConundrumError = (content: ConundrumErrorVariant): ConundrumError | undefined => {
    if (typeof content.content === "object" && "msg" in content.content) {
        return content.content
    } else {
        return undefined
    }
}


export const PreviewLevelErrorReport = ({ debounceTimeout, showWebviewAction, additionalComponents, id, mdx, resetErrorBoundary, conundrumError }: Partial<FallbackProps> & Pick<MdxContentProps, "debounceTimeout" | "showWebviewAction" | "additionalComponents"> & { id: string, mdx: string, conundrumError?: ConundrumErrorVariant }): ReactNode => {
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
        dispatch(clearConundrumError())
    })

    const error = conundrumError ? getConundrumError(conundrumError) : null


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
                    <div className="font-bold">{error?.msg ?? "Error"}</div>
                    <div className="text-muted-foreground text-sm text-center">
                        <InlineMdxContent
                            mdx={error?.details ?? "Fluster encountered an error while attempting to parse this document. Please double-check your syntax."}
                        />
                    </div>
                </div>
            </motion.div>
        </div>
    )
}


PreviewLevelErrorReport.displayName = "PreviewLevelErrorReport"
