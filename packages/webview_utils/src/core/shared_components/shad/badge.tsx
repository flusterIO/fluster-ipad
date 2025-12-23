import React, { HTMLProps, type ReactNode } from 'react'
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from '@/utils/cn';
import { AnyWebviewAction } from '@/utils/types/any_window_event';
import { sendToSwift } from '@/utils/bridge/send_to_swift';

const badgeVariants = cva(
    "inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 [&>svg]:size-3 gap-1 [&>svg]:pointer-events-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive transition-[color,box-shadow] overflow-hidden",
    {
        variants: {
            variant: {
                default:
                    "border-transparent bg-primary text-primary-foreground [a&]:hover:bg-primary/90",
                secondary:
                    "border-transparent bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90",
                destructive:
                    "border-transparent bg-destructive text-white [a&]:hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
                outline:
                    "text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground",
            },
        },
        defaultVariants: {
            variant: "default",
        },
    }
);

export interface BadgeProps extends HTMLProps<HTMLSpanElement> {
    variant?: "default" | "secondary" | "destructive" | "outline"
    className?: string
}

export const Badge = ({
    variant,
    className,
    ...props
}: BadgeProps): ReactNode => {
    return (
        <span
            data-slot="badge"
            className={cn(badgeVariants({ variant }), className)}
            {...props}
        />
    )
}


Badge.displayName = "Badge"


interface TagBadgeProps extends BadgeProps {
    clickAction: AnyWebviewAction
    taggableValue: string
}

export const TaggableBadge = ({
    clickAction,
    taggableValue,
    ...props
}: TagBadgeProps): ReactNode => {
    return (
        <Badge
            {...props}
            onClick={() => {
                sendToSwift(clickAction, taggableValue)
            }}
        >
            {taggableValue}
        </Badge>
    )
}


TaggableBadge.displayName = "TaggableBadge"
