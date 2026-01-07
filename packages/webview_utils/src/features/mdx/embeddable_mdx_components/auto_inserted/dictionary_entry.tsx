import React, { type ReactNode } from 'react'


interface DictionaryEntryProps {
    label: string
    children: ReactNode
}

export const DictionaryEntry = ({ label, children }: DictionaryEntryProps): ReactNode => {
    return (
        <div className="w-full max-w-[1080px] h-fit bg-primary/30 p-4">
            <h2 className="text-4xl font-serif font-medium tracking-tight text-foreground">{label}</h2>
            {children}
        </div>
    )
}


DictionaryEntry.displayName = "DictionaryEntry"
