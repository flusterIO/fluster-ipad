import { BibliographyEntryComponent } from '#/bibliography/presentation/bibliography_entry';
import { type GlobalWebviewStateDeepNullable } from '#/webview_global_state/cross_language_state_types';
import { type GlobalAppState } from '#/webview_global_state/store'
import { H3 } from '@/shared_components/typography/typography';
import React, { type ReactNode } from 'react'
import { connect } from 'react-redux';

const connector = connect((state: GlobalWebviewStateDeepNullable) => ({
    citations: state.editor.citations
}))

interface NoteBibliographyProps {
    citations: GlobalAppState["editor"]["citations"]
}

export const NoteBibliography = connector(({ citations }: NoteBibliographyProps): ReactNode => {
    if (!citations.length) {
        return null
    }
    return (
        <div className="w-full max-w-full">
            <H3>Bibliography</H3>
            <div className="w-full">
                {citations.map((cit) => <BibliographyEntryComponent key={cit.citation_key} entry={cit} />)}
            </div>
        </div>
    )
})


NoteBibliography.displayName = "NoteBibliography"
