import { Provider } from 'react-redux'
import React, { type ReactNode } from 'react'
import { type GlobalStatePersistor, type GlobalStateStore } from './store'
import { PersistGate } from 'redux-persist/integration/react'
import { LoadingComponent } from '@/shared_components/loading/loading_component'
import { WebviewClient } from '../webview_container/data/webview_client'
import { sendToSwift } from '@/utils/bridge/send_to_swift'
import { type ReduxStateLoadedEvent, WebviewContainerEvents } from '@/code_gen/typeshare/fluster_core_utilities'
import { GlobalStateListeners } from './container/use_global_state_resize_listener'
import { SplitviewEditorNotificationBanner } from '#/notifications/splitview_editor_notification_banner/splitview_editor_notification_banner'


interface MdxEditorGlobalProviderProps {
    children: ReactNode
    store: GlobalStateStore
    persistor: GlobalStatePersistor
}
declare global {

    interface WindowEventMap {
        "redux-state-loaded": CustomEvent<undefined>;
    }
}



export const MdxEditorGlobalProvider = ({ children, store, persistor }: MdxEditorGlobalProviderProps): ReactNode => {
    const onBeforeStateLoad = (): void => {
        const state = store.getState()
        WebviewClient.applyGlobalState(state)
        const data: ReduxStateLoadedEvent = {
            note_id: state.editor.note_id
        }
        sendToSwift(WebviewContainerEvents.ReduxStateLoaded, JSON.stringify(data))
        window.dispatchEvent(new CustomEvent("redux-state-loaded", {}))
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
                <SplitviewEditorNotificationBanner />
                <GlobalStateListeners />
            </PersistGate>
        </Provider>
    )
}


MdxEditorGlobalProvider.displayName = "MdxEditorGlobalProvider"
