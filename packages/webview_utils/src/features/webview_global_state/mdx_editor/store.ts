import { combineReducers, configureStore } from '@reduxjs/toolkit'
import editorReducer from './state/editor_state_slice'
import storage from "redux-persist/lib/storage";
import { persistStore, persistReducer, type PersistConfig } from "redux-persist"
import webviewContainerReducer from '../shared/webview_container_global_state/webview_container_slice';
import { type WebviewContainerState } from '@/code_gen/typeshare/fluster_core_utilities';
import { darkModeListenerMiddleware } from '../shared/webview_container_global_state/side_effects/dark_mode_side_effect';
import { type EditorState } from '@codemirror/state';
import { emptyValueListenerMiddleware } from "./side_effects/empty_editor_value_side_effect"
import { emptyParsedValueListenerMiddleware } from "./side_effects/empty_parsed_value_side_effect"

const rootReducer = combineReducers({
    editor: editorReducer,
    container: webviewContainerReducer
});

export type MdxEditorAppState = ReturnType<typeof rootReducer>

const persistConfig: PersistConfig<MdxEditorAppState> = {
    key: "fluster_mdx_editor",
    storage,
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
