import { InlineMdxContent } from '#/mdx/components/inline_mdx_content'
import { type ConundrumError } from '@/code_gen/typeshare/conundrum'
import React, { type ReactNode } from 'react'



interface ConundrumErrorReportItemProps {
    item: ConundrumError
    idx: number
}

export const ConundrumErrorReportItem = ({ item, idx }: ConundrumErrorReportItemProps): ReactNode => {
    return (
        <div className="w-full grid grid-cols-[auto_1fr] gap-x-4 px-3">
            <div className="w-fit h-fit pt-1">
                <div className="text-sm text-fd-card-foreground/70">{idx + 1}</div>
            </div>
            <div>
                <InlineMdxContent
                    mdx={item.msg}
                    className="[&_p]:font-bold"
                />
                {item.details ? (
                    <InlineMdxContent
                        mdx={item.details}
                        className="prose-sm"
                    /* className="[&_p]:font-bold" */
                    />
                ) : null}
            </div>
        </div>
    )
}


ConundrumErrorReportItem.displayName = "ConundrumErrorReportItem"
