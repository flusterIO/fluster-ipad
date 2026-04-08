import { type ConundrumError } from '@/code_gen/typeshare/conundrum'
import React, { type ReactNode } from 'react'
import { ConundrumErrorReportItem } from './conundrum_error_report_item'



interface ConundrumErrorsReportProps {
    errors: ConundrumError[]
}

export const ConundrumErrorsReport = (props: ConundrumErrorsReportProps): ReactNode => {
    return (
        <div className="w-full h-fit mt-2">
            <div className="w-full h-fit max-h-[450px] overflow-x-hidden overflow-y-auto scroll-pb-4">
                {props.errors.map((item, i) => {
                    return (
                        <ConundrumErrorReportItem idx={i} key={`${item.msg}-${item.details ?? ""}`} item={item} />
                    )
                })}
            </div>
        </div>
    )
}


ConundrumErrorsReport.displayName = "ConundrumErrorsReport"
