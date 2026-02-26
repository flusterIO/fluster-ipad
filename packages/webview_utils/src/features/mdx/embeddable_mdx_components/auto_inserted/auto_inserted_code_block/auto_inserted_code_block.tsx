import { cn } from '@/utils/cn'
import React, { HTMLProps, type ReactNode } from 'react'



export const AutoInsertedCodeBlock = ({ className, children, ...props }: HTMLProps<HTMLPreElement>): ReactNode => {
    return (
        <pre className={cn("rounded", className)} {...props}>
            {children}
        </pre>
    )
}


AutoInsertedCodeBlock.displayName = "AutoInsertedCodeBlock"
