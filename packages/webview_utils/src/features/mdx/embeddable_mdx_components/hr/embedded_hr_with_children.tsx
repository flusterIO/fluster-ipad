import React, { type ReactNode } from 'react'
import { hrPropsSchema } from './hr_props_schema'


interface EmbeddedHrWithChildrenProps {
    children: ReactNode
}


export const EmbeddedHrWithChildren = (props: EmbeddedHrWithChildrenProps): ReactNode => {
    const data = hrPropsSchema.parse(props)
    const content = "children" in data ? data.children : "content" in data ? data.content : ""
    if (!content) {
        return <hr />
    }
    return (
        <div
            className="w-full grid grid-cols-[1fr_auto_1fr] gap-x-4 place-items-center mt-6"
        >
            <div className="bg-border h-[2px] w-full" />
            <div className="text-muted-foreground text-sm my-0! h-fit text-center [&>*]:text-center [&>*]:text-muted-foreground! [&>p]:text-sm [&>*]:my-0! [&>*]:h-fit! max-w-[min(250px,50vw)]">{content}</div>
            <div className="bg-border h-[2px] w-full" />
        </div>
    )
}


EmbeddedHrWithChildren.displayName = "EmbeddedHrWithChildren"
