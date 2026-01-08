import { H3 } from "@/shared_components/typography/typography"
import { cn } from "@/utils/cn"
import React, { ReactNode } from "react"

export const Subtitle = ({ children, className }: { children: ReactNode, className?: string }): ReactNode => {
    return (
        <H3 className={cn("w-full text-muted-foreground tracking-wide", className)}>{children}</H3>
    )
}
