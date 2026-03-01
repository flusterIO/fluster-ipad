import React, { type ReactNode } from 'react'



interface EmbeddedHrWithChildrenProps {
    children: ReactNode
}

export const EmbeddedHrWithChildren = ({ children }: EmbeddedHrWithChildrenProps): ReactNode => {
    return (
        <div
            className="w-full grid grid-cols-[1fr_auto_1fr] gap-x-4"
        >
            <div className="bg-border h-[2px] w-full" />
            <div className="[&_*]:text-muted-foreground [&_*]:text-sm max-w-[min(350px,50vw)]">{children}</div>
            <div className="bg-border h-[2px] w-full" />
        </div>
    )
}


EmbeddedHrWithChildren.displayName = "EmbeddedHrWithChildren"
