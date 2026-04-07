import React, { type ReactNode } from 'react'
import { type BlockMathResult } from "@/code_gen/typeshare/conundrum"



interface AutoInsertedInlineMathProps {
    data: BlockMathResult
    children: ReactNode
}

/**
 * @deprecated -- oined with other math component.
 */
export const AutoInsertedInlineMath = ({ data, children }: AutoInsertedInlineMathProps): ReactNode => {
    console.log("data: ", data)
    return (
        <div className="inline w-full my-6">{children}</div>
    )
}


