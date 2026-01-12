import React, { type ReactNode } from 'react'



interface AppErrorViewProps {
    error: string | null
}

export const AppErrorView = ({ error }: AppErrorViewProps): ReactNode => {
    if (error === null) {
        return
    }
    return (
        <div>{error}</div>
    )
}


AppErrorView.displayName = "AppErrorView"
