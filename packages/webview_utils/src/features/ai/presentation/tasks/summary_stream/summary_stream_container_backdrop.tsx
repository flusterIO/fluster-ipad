import React, { type ReactNode } from 'react'



interface SummaryStreamContainerBackdropProps {
    children: ReactNode
}

export const SummaryStreamContainerBackdrop = ({ children }: SummaryStreamContainerBackdropProps): ReactNode => {
    return (
        <div
            className="bg-background/70 fixed top-0 left-0 right-0 bottom-0 w-screen h-screen px-4 py-6"
        >{children}</div>
    )
}


SummaryStreamContainerBackdrop.displayName = "SummaryStreamContainerBackdrop"
