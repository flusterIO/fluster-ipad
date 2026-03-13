import { combineReducers, configureStore, type Reducer } from '@reduxjs/toolkit'
import editorReducer from './mdx_editor/state/editor_state_slice'
import { persistStore, persistReducer, type PersistConfig } from "redux-persist"
import webviewContainerReducer from './container/webview_container_global_state/webview_container_slice';
import aiReducer from './ai_state/ai_state_slice';
import notificationReducer from './notification_state/notification_state_slice';
import noteDetailsReducer from "./note_details_state/note_details_slice"
import mediaReducer from './media_state/media_state_slice';
import dictionaryReducer from './dictionary_state/dictionary_slice';
import { type EditorState, type WebviewContainerState } from '@/code_gen/typeshare/fluster_core_utilities';
import { darkModeListenerMiddleware } from './container/webview_container_global_state/side_effects/dark_mode_side_effect';
import { emptyValueListenerMiddleware } from "./mdx_editor/side_effects/empty_editor_value_side_effect"
import autoMergeLevel2 from 'redux-persist/lib/stateReconciler/autoMergeLevel2'
import { emptyParsedValueListenerMiddleware } from "./mdx_editor/side_effects/empty_parsed_value_side_effect"
import { editorValueChangeListenerMiddleware } from './mdx_editor/side_effects/editor_value_update_side_effect';
import storage from 'redux-persist-indexeddb-storage';
import { editorBibValueChangeListenerMiddleware } from './mdx_editor/side_effects/editor_bib_value_side_effect';
import { type GlobalWebviewStateDeepNullable, type GlobalWebviewStateDeepNullableReducer, type GlobalWebviewStateNullable } from './cross_language_state_types';
import { copyStringToClipboard } from '@/utils/string_utils';



const rootReducer = combineReducers({
    editor: editorReducer,
    container: webviewContainerReducer,
    ai: aiReducer,
    media: mediaReducer,
    notifications: notificationReducer,
    note_details: noteDetailsReducer,
    dictionary: dictionaryReducer
} satisfies GlobalWebviewStateDeepNullableReducer);


const persistConfig: PersistConfig<GlobalWebviewStateDeepNullable> = {
    key: "fluster_mdx_editor",
    /* eslint-disable-next-line  -- I've been doing this 10 years and I don't know how to declare this module properly. */
    storage: storage("fluster"),
    stateReconciler: autoMergeLevel2,
    debug: import.meta.env.DEV,
    blacklist: ["editor.note_id", "container.wasm_loaded", "container.size"] satisfies (`editor.${keyof EditorState}` | `container.${keyof WebviewContainerState}`)[],
};



const persistedReducer = persistReducer<GlobalWebviewStateDeepNullable>(persistConfig, rootReducer);
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
                .prepend(editorBibValueChangeListenerMiddleware.middleware)
                .prepend(emptyParsedValueListenerMiddleware.middleware)
        ,
    });
    if (import.meta.env.FLUSTER_PROD_BUILD !== "true") {
        window.store = store
    }
    const persistor = persistStore(store);
    return { store, persistor };
};



export type GlobalAppState = ReturnType<typeof rootReducer>
export type GlobalStateStore = ReturnType<typeof createFlusterStore>["store"]
export type GlobalStatePersistor = ReturnType<typeof createFlusterStore>["persistor"]
export type AppDispatch = GlobalStateStore["dispatch"]

declare global {

    interface Window {
        "store": GlobalStateStore | undefined
        "copyState"?: () => Promise<void>
    }
}



