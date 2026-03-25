import React, { useEffect, type ReactNode } from 'react'
import { createFlusterStore } from '#/webview_global_state/store';
import { handleSwiftActionWrapper, handleSwiftBufferActionWrapper } from '#/webview_global_state/window_methods';
import { MdxEditorGlobalProvider } from '#/webview_global_state/provider';
import { WebViewContainer } from '#/webview_container/presentation/webview_container';
import { AIGeneratedContainerDevWrapper } from './development_wrapped_components/ai_generated_container_dev_wrapper';
import { WebviewEnvironment } from '@/code_gen/typeshare/fluster_core_utilities';

const storeData = createFlusterStore();

window.handleSwiftAction = handleSwiftActionWrapper(storeData.store);

window.handleSwiftBufferAction = handleSwiftBufferActionWrapper(
    storeData.store,
);


const CurrentDevelopmentComponent = (): ReactNode => {
    return (
        <div className="w-full h-full flex flex-col justify-center items-center">
            <AIGeneratedContainerDevWrapper />
        </div>
    )
}



// Don't touch this, just mess with the component above.
export const DevelopmentTestPage = (): ReactNode => {
    useEffect(() => {
        document.body.classList.add(WebviewEnvironment.MacOS)
    }, [])
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
