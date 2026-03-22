import React, { type ReactNode } from 'react'
import { createFlusterStore } from '#/webview_global_state/store';
import { handleSwiftActionWrapper, handleSwiftBufferActionWrapper } from '#/webview_global_state/window_methods';
import { MdxEditorGlobalProvider } from '#/webview_global_state/provider';
import { AiParsePhase1DevContainer } from './development_wrapped_components/fluster_ai_parse_pending_container_dev_wrapper';
import { WebViewContainer } from '#/webview_container/presentation/webview_container';


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
            <WebViewContainer
                className="py-12"
            >
                <AiParsePhase1DevContainer />
            </WebViewContainer>
        </MdxEditorGlobalProvider>
    )
}


DevelopmentTestPage.displayName = "DevelopmentTestPage"
