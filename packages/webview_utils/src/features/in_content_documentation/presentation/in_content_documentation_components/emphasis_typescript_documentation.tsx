import React, { type ReactNode } from 'react'
import { emphasisToBackgroundClasses, getEmphasisOptions } from '../../../mdx/embeddable_mdx_components/schemas/emphasis_schema'
import { cn } from '../../../../core/utils/cn'

export const InContentDocsEmphasisTypeList = (): ReactNode => {
    const options = getEmphasisOptions()
    return (
        <div
            className="w-fit h-fit flex flex-col justify-center items-center"
        >
            {options.map((o) => {
                return (
                    <div className="grid grid-cols-[auto_1fr] gap-x-4" key={o}>
                        <div
                            className={cn("w-8 h-8 rounded border", emphasisToBackgroundClasses(o))}
                        />
                        <div className="w-full">
                            {o}
                        </div>
                    </div>
                )
            })}
        </div>
    )
}


InContentDocsEmphasisTypeList.displayName = "InContentDocsEmphasisTypeList"
