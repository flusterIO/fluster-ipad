import { Provider } from 'react-redux'
import React, { type ReactNode } from 'react'
import { type MdxEditorPersistor, type MdxEditorStore } from './store'
import { PersistGate } from 'redux-persist/integration/react'
import { LoadingComponent } from '@/shared_components/loading/loading_component'


interface MdxEditorGlobalProviderProps {
    children: ReactNode
    store: MdxEditorStore
    persistor: MdxEditorPersistor
}

export const MdxEditorGlobalProvider = ({ children, store, persistor }: MdxEditorGlobalProviderProps): ReactNode => {
    return (
        <Provider
            store={store}
        >
            <PersistGate
                persistor={persistor}
                loading={<div className="w-full h-full flex flex-col justify-center items-center p-3">
                    <LoadingComponent />
                </div>}
            >
                {children}
            </PersistGate>
        </Provider>
    )
}


MdxEditorGlobalProvider.displayName = "MdxEditorGlobalProvider"
