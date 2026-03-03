import { TriangleAlert } from 'lucide-react'
import React, { type ReactNode } from 'react'


export const PreviewLevelErrorReport = (): ReactNode => {
    return (
        <div className="w-full h-full flex flex-col justify-center items-center">
            <div
                className="w-full max-w-[540px] flex flex-col justify-center items-center gap-y-6"
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
            </div>
        </div>
    )
}


PreviewLevelErrorReport.displayName = "PreviewLevelErrorReport"
