/* import { rspc } from '@/app/rspc_client' */
import { cn } from '@/utils/shad_utils'
import React, { type ReactNode } from 'react'



interface InlineCdrmContentProps {
   content: string
    className?: string
    em?: "span" | "div"
}

export const InlineCdrmContent = ({className, content, em="span"}: InlineCdrmContentProps): ReactNode => {
    /* const {} = rspc.useMutation("") */
    if (em === "span") {
return (
    <span className={cn("inline-block", className)}/>
)
    } else {
return (
    <div className={cn("inline-block", className)}/>
)
    }
}


InlineCdrmContent.displayName = "InlineCdrmContent"
