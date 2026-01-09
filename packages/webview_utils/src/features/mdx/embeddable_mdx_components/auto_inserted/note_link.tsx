import React, { type ReactNode } from 'react'
import { sendToSwift } from '../../../../core/utils/bridge/send_to_swift'
import { MdxPreviewWebviewActions } from '../../../../core/code_gen/typeshare/fluster_core_utilities'



interface NoteLinkProps {
    /// The user defined id of the note to link to, not the DOM id or the database id.
    id: string
    children: ReactNode
}

export const NoteLink = ({ id, children }: NoteLinkProps): ReactNode => {
    return (
        <a role="button"
            onClick={() => {
                sendToSwift(MdxPreviewWebviewActions.ViewNoteByUserDefinedId, id)
            }}
            className="text-primary font-semibold"
        >{children}</a>
    )
}


NoteLink.displayName = "NoteLink"
