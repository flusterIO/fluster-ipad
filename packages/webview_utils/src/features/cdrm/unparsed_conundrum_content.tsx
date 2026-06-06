import { ConundrumStateActions, type ParseConundrumContentRequest } from '@/code_gen/typeshare/fluster_core_utilities';
import { sendToSwift } from '@/utils/bridge/send_to_swift';
import React, { useEffect, type ReactNode } from 'react'



export interface UnparsedConundrumContentProps {
    /**
     * Some id that can be used to identify this content, ideally depepdnent on something that won't change like a note id or some shit.
     * This ID must exist in the DOM when the component is being rendered in order for it to be properly replaced.
     */
    DOMId: string
    content: string
}


export const UnparsedConundrumContent = ({ DOMId, content }: UnparsedConundrumContentProps): ReactNode => {

    useEffect(() => {
        sendToSwift(ConundrumStateActions.ParseConundrumContent, JSON.stringify({
            content: content,
            DOMId
        } satisfies ParseConundrumContentRequest))
    }, [])
    return null
}


UnparsedConundrumContent.displayName = "UnparsedConundrumContent"
