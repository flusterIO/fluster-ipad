import { XCircleIcon } from 'lucide-react'
import React, { type ReactNode } from 'react'
import { ZodIssue } from 'zod'


interface InContentErrorReportItemProps {
    item: ZodIssue
}

export const InContentErrorReportItem = ({ item }: InContentErrorReportItemProps): ReactNode => {
    console.log("item: ", item)
    return (
        <div className="w-full grid grid-cols-[36px_1fr] p-2 place-items-center">
            <XCircleIcon className="w-3 h-3 stroke-danger" />
            <div className="w-full">
                {item.message}
            </div>
        </div >
    )
}


InContentErrorReportItem.displayName = "InContentErrorReportItem"
