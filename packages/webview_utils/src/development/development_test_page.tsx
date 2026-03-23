import React, { type ReactNode } from 'react'
import { createFlusterStore } from '#/webview_global_state/store';
import { handleSwiftActionWrapper, handleSwiftBufferActionWrapper } from '#/webview_global_state/window_methods';
import { MdxEditorGlobalProvider } from '#/webview_global_state/provider';
import { WebViewContainer } from '#/webview_container/presentation/webview_container';
import { NoteDetailsDevelopmentWrapper } from './development_wrapped_components/note_details_development_wrapper';

const storeData = createFlusterStore();

window.handleSwiftAction = handleSwiftActionWrapper(storeData.store);

window.handleSwiftBufferAction = handleSwiftBufferActionWrapper(
    storeData.store,
);


const CurrentDevelopmentComponent = (): ReactNode => {
    return (
        <NoteDetailsDevelopmentWrapper />
    )
}



// Don't touch this, just mess with the component above.
export const DevelopmentTestPage = (): ReactNode => {
    return (
        <MdxEditorGlobalProvider
            {...storeData}
        >
            <WebViewContainer
                className="py-12"
            >
                <CurrentDevelopmentComponent />
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    )
}


DevelopmentTestPage.displayName = "DevelopmentTestPage"
