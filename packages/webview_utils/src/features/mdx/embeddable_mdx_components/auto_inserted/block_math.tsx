import React, { type ReactNode } from 'react'
import { type BlockMathResult } from "@/code_gen/typeshare/conundrum"



interface AutoInsertedBlockMathProps {
    data: BlockMathResult
    children: ReactNode
}

/**
 * @deprecated - Joined with other math component.
 */
export const AutoInsertedBlockMath = ({ data, children }: AutoInsertedBlockMathProps): ReactNode => {
    console.log("data: ", data)
    return (
        <div className="block w-full my-6">{children}</div>
    )
}


