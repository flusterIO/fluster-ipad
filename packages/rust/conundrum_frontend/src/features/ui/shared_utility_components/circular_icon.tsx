import { cn } from '@/utils/shad_utils'
import React, { type FC, type ReactNode } from 'react'



interface CircularIconProps {
    icon: FC<{ className: string }>
    classes?: {
        container?: string
        icon?: string
    }
    destructive?: boolean
}

export const CircularIcon = ({ icon: Icon, classes = {}, destructive }: CircularIconProps): ReactNode => {
    return (
        <div className={cn("w-fit h-fit rounded-full p-3", classes.container, destructive && "bg-destructive/50 text-destructive-foreground")}>
            <Icon className={cn("w-10 h-10", classes.icon)} />
        </div>
    )
}


CircularIcon.displayName = "CircularIcon"
