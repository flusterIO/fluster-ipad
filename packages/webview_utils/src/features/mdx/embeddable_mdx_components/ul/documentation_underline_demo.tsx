import React, { type ReactNode } from 'react'
import { emphasisOptions } from '../schemas/emphasis_schema'
import { Ul } from './ul'

export const InContentUnderlineDocumentationDemo = (): ReactNode => {
    return (
        <div className="w-full h-fit flex flex-col justify-start items-start gap-y-1">
            {emphasisOptions.map((opt) => {
                const props = {
                    [opt]: true
                }
                return <div>This is <Ul {...props}>{opt}</Ul> and <Ul {...props} thick>thick</Ul> underlined.</div>
            })}
        </div>
    )
}


InContentUnderlineDocumentationDemo.displayName = "InContentUnderlineDocumentationDemo"
