import { H3 } from "@/shared_components/typography/typography"
import { cn } from "@/utils/cn"
import React, { type ReactNode } from "react"

export const Subtitle = ({ children, className }: { children: ReactNode, className?: string }): ReactNode => {
    return (
        <h3 className={cn("w-full not-prose text-muted-foreground tracking-wide text-2xl font-semibold my-4", className)}>{children}</h3>
    )
}
