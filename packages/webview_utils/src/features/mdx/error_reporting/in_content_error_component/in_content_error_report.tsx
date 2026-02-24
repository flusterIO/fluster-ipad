import React, { useState, type ReactNode } from 'react'
import { FallbackProps } from 'react-error-boundary'
import { ZodError } from 'zod'
import { InContentErrorReportItem } from './in_content_error_report_item'
import { CircleX, TriangleAlert, XIcon } from 'lucide-react'
import { AnimatePresence, motion } from 'framer-motion'


export const InContentErrorReport = (props: FallbackProps & { componentName: string }): ReactNode => {
    const [show, setShow] = useState(true)
    if (!(props.error instanceof ZodError) || props.error.isEmpty) {
        return null
    }
    return (
        <AnimatePresence>
            {show ? (
                <motion.div
                    className="w-full max-w-[768px] mx-auto block border border-destructive/30 rounded-lg pb-2"
                    key={props.error.toString()}
                    exit={{
                        height: 0,
                        opacity: 0,
                    }}
                    onClick={() => setShow(false)}
                >
                    <div
                        className="w-full flex flex-col justify-center items-center @[540px]:grid @[540px]:grid-cols-[auto_1fr_auto] gap-4 relative py-4 px-6"
                    >
                        <div className="rounded-lg bg-destructive/20 p-3">
                            <TriangleAlert className="stroke-destructive" />
                        </div>
                        <div className="w-full flex flex-col justify-center items-center @[540px]:inline-block">
                            <div className="font-bold">Error</div>
                            <div className="text-muted-foreground text-sm">{`There ${props.error.issues.length == 1 ? "is" : "are"} ${props.error.issues.length} issue${props.error.issues.length > 1 ? "s" : ""} in the ${props.componentName} component.`}</div>
                        </div>
                        <div
                            className="flex flex-row justify-center items-center gap-x-2"
                        >
                            <div className="text-destructive flex flex-row justify-center items-center flex-nowrap gap-x-1">
                                <CircleX className="stroke-destructive w-3 h-3" />
                                <span className="text-sm">
                                    {props.error.issues.length}
                                </span>
                            </div>
                        </div>
                    </div>
                    <div className="w-full">
                        {props.error.issues.map((issue) => {
                            return (
                                <InContentErrorReportItem item={issue} key={`${issue.code}-${issue.path}`} />
                            )
                        })}
                    </div>
                </motion.div>) : null
            }
        </AnimatePresence>
    )
}


InContentErrorReport.displayName = "InContentErrorReport"
