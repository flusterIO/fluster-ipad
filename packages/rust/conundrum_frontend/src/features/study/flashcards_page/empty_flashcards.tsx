import { AppPaths } from '#/navigation/app_paths'
import { SearchSlashIcon } from 'lucide-react'
import React, { type ReactNode } from 'react'
import { Link } from 'react-router'

export const EmptyFlashcards = (): ReactNode => {
    return (
        <div className="w-fit h-fit max-w-87.5 rounded bg-fd-card border mx-auto flex flex-col justify-center items-center gap-y-4 text-fd-card-foreground p-3">
            <div className="bg-destructive/50 text-destructive-foreground rounded-full p-2">
                <SearchSlashIcon className="w-12 h-12" />
            </div>
            <div className="text-lg font-semibold">No Flashcards Found</div>
            <div className="text-center">Click <Link className="text-primary" to={AppPaths.aiGenerateFlashcard}>here</Link> to generate new flashcards with AI.</div>
        </div>
    )
}


EmptyFlashcards.displayName = "EmptyFlashcards"
