import React, { type ReactNode } from 'react'
import { createFlusterStore } from '#/webview_global_state/store';
import { handleSwiftActionWrapper, handleSwiftBufferActionWrapper } from '#/webview_global_state/window_methods';
import { MdxEditorGlobalProvider } from '#/webview_global_state/provider';
import { NoteDetailsDevelopmentWrapper } from './development_wrapped_components/note_details_development_wrapper';


const testContent = `
# Test Content

<FlusterAIParsePending>
Summarize this note please.
</FlusterAIParsePending>
`

const storeData = createFlusterStore();

window.handleSwiftAction = handleSwiftActionWrapper(storeData.store);

window.handleSwiftBufferAction = handleSwiftBufferActionWrapper(
    storeData.store,
);



export const DevelopmentTestPage = (): ReactNode => {
    return (
        <MdxEditorGlobalProvider
            {...storeData}
        >
            <NoteDetailsDevelopmentWrapper />
        </MdxEditorGlobalProvider>
    )
}


DevelopmentTestPage.displayName = "DevelopmentTestPage"
