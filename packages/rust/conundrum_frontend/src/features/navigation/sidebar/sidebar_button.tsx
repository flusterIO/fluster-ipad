import { type HTMLProps } from '@base-ui/react';
import React, { type FC, type ReactNode } from 'react'



export type PermanentSidebarButtonProps = {
    icon: FC<{ className?: string }>
} & ({
    onClick?: () => void
    href?: undefined;
    id: string
} | {
    onClick?: undefined;
    href?: string
});

export const PermanentSidebarButton = ({ onClick, href, icon: Icon }: PermanentSidebarButtonProps): ReactNode => {
    const props: HTMLProps<HTMLAnchorElement> = {
        ...(href ? { href } : { role: "button", onClick }),
    }
    return (
        <a {...props} className="grid place-items-center">
            <Icon className="w-6 h-6" />
        </a>
    )

}


PermanentSidebarButton.displayName = "PermanentSidebarButton"
