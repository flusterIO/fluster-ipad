import React, { type ReactNode } from 'react'

interface EmojiDocsDemoProps {
    children: ReactNode
    name: string
}

export const EmojiDocsDemo = ({ children, name }: EmojiDocsDemoProps): ReactNode => {
    return (
        <div className="w-full h-full flex flex-col justify-center items-center gap-0 p-0 bg-fd-card">
            <div className="flex-grow-1 p-4 bg-background flex flex-col justify-center items-center">
                {children}
            </div>
            <div className="text-center w-full px-4 py-2 text-sm">{name}</div>
        </div>
    )
}


EmojiDocsDemo.displayName = "EmojiDocsDemo"
