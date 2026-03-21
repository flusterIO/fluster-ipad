import React, { type ReactNode } from 'react'
import { type BlockMathResult } from "@/code_gen/typeshare/conundrum"



interface AutoInsertedBlockMathProps {
    data: BlockMathResult
    children: ReactNode
}

export const AutoInsertedBlockMath = ({ data, children }: AutoInsertedBlockMathProps): ReactNode => {
    console.log("data: ", data)
    return (
        <div className="block w-full my-6">{children}</div>
    )
}


AutoInsertedBlockMath.displayName = "AutoInsertedBlockMath"
