import React, { type HTMLProps, type ReactNode } from 'react'



export const ExternalLink = (props: HTMLProps<HTMLAnchorElement>): ReactNode => {
    return (
        <a {...props} role="button" />
    )
}


ExternalLink.displayName = "ExternalLink"
