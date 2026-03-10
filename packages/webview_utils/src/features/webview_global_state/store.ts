import { combineReducers, configureStore } from '@reduxjs/toolkit'
import editorReducer from './mdx_editor/state/editor_state_slice'
import { persistStore, persistReducer, type PersistConfig } from "redux-persist"
import webviewContainerReducer from './container/webview_container_global_state/webview_container_slice';
import aiReducer from './ai_state/ai_state_slice';
import notificationReducer from './notification_state/notification_state_slice';
import mediaReducer from './media_state/media_state_slice';
import { type GlobalWebviewState, type WebviewContainerState } from '@/code_gen/typeshare/fluster_core_utilities';
import { darkModeListenerMiddleware } from './container/webview_container_global_state/side_effects/dark_mode_side_effect';
import { type EditorState } from '@codemirror/state';
import { emptyValueListenerMiddleware } from "./mdx_editor/side_effects/empty_editor_value_side_effect"
import autoMergeLevel2 from 'redux-persist/lib/stateReconciler/autoMergeLevel2'
import { emptyParsedValueListenerMiddleware } from "./mdx_editor/side_effects/empty_parsed_value_side_effect"
import { editorValueChangeListenerMiddleware } from './mdx_editor/side_effects/editor_value_update_side_effect';
import storage from 'redux-persist-indexeddb-storage';

const rootReducer = combineReducers({
    editor: editorReducer,
    container: webviewContainerReducer,
    ai: aiReducer,
    media: mediaReducer,
    notifications: notificationReducer
} satisfies Record<keyof GlobalWebviewState, unknown>);

export type MdxEditorAppState = ReturnType<typeof rootReducer>

const persistConfig: PersistConfig<MdxEditorAppState> = {
    key: "fluster_mdx_editor",
    /* eslint-disable-next-line  -- I've been doing this 10 years and I don't know how to declare this module properly. */
    storage: storage("fluster"),
    stateReconciler: autoMergeLevel2,
    debug: import.meta.env.DEV,
    blacklist: ["editor.note_id", "container.wasm_loaded", "container.size"] as (`editor.${keyof EditorState}` | `container.${keyof WebviewContainerState}`)[],
};



const persistedReducer = persistReducer(persistConfig, rootReducer);
// Add custom middleware here if needed
// Export a factory instead of a singleton
export const createFlusterStore = () => {
    const isProd = import.meta.env.FLUSTER_PROD_BUILD === "true";
    const store = configureStore({
        reducer: persistedReducer,
        devTools: !isProd,
        // preloadedState: initialMdxEditorState,
        middleware: (getDefaultMiddleware) =>
            getDefaultMiddleware({
                // Required to stop Redux Toolkit from complaining about non-serializable persist actions
                serializableCheck: {
                    ignoredActions: ["persist/PERSIST", "persist/REHYDRATE", "persist/REGISTER"],
                },
            })
                .prepend(darkModeListenerMiddleware.middleware)
                .prepend(emptyValueListenerMiddleware.middleware)
                .prepend(editorValueChangeListenerMiddleware.middleware)
                .prepend(emptyParsedValueListenerMiddleware.middleware)
        ,
    });
    if (import.meta.env.FLUSTER_PROD_BUILD !== "true") {
        window.store = store
    }
    const persistor = persistStore(store);
    return { store, persistor };
};



export type MdxEditorStore = ReturnType<typeof createFlusterStore>["store"]
export type MdxEditorPersistor = ReturnType<typeof createFlusterStore>["persistor"]
export type AppDispatch = MdxEditorStore["dispatch"]

declare global {

    interface Window {
        "store": MdxEditorStore | undefined
    }
}
