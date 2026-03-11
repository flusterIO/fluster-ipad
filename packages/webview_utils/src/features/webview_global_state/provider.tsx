import { Provider } from 'react-redux'
import React, { type ReactNode } from 'react'
import { type MdxEditorPersistor, type MdxEditorStore } from './store'
import { PersistGate } from 'redux-persist/integration/react'
import { LoadingComponent } from '@/shared_components/loading/loading_component'
import { WebviewClient } from '../webview_container/data/webview_client'
import { sendToSwift } from '@/utils/bridge/send_to_swift'
import { type ReduxStateLoadedEvent, WebviewContainerEvents } from '@/code_gen/typeshare/fluster_core_utilities'


interface MdxEditorGlobalProviderProps {
    children: ReactNode
    store: MdxEditorStore
    persistor: MdxEditorPersistor
}



export const MdxEditorGlobalProvider = ({ children, store, persistor }: MdxEditorGlobalProviderProps): ReactNode => {
    const onBeforeStateLoad = (): void => {
        console.log("Before state load")
        const state = store.getState()
        WebviewClient.applyGlobalState(state)
        const data: ReduxStateLoadedEvent = {
            note_id: state.editor.note_id
        }
        sendToSwift(WebviewContainerEvents.ReduxStateLoaded, JSON.stringify(data))
    }
    return (
        <Provider
            store={store}
        >
            <PersistGate
                persistor={persistor}
                loading={<div className="w-full h-full flex flex-col justify-center items-center p-3">
                    <LoadingComponent />
                </div>}
                onBeforeLift={onBeforeStateLoad}
            >
                {children}
            </PersistGate>
        </Provider>
    )
}


MdxEditorGlobalProvider.displayName = "MdxEditorGlobalProvider"
