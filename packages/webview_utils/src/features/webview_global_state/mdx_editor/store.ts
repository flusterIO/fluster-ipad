import { combineReducers, configureStore, createListenerMiddleware } from '@reduxjs/toolkit'
import editorReducer from './state/editor_state_slice'
import storage from "redux-persist/lib/storage";
import { persistStore, persistReducer, type PersistConfig } from "redux-persist"
import webviewContainerReducer, { setDarkMode } from '../shared/webview_container_global_state/webview_container_slice';
import { type WebviewContainerState, type WebviewEnvironment } from '@/code_gen/typeshare/fluster_core_utilities';
import { darkModeListenerMiddleware } from '../shared/webview_container_global_state/side_effects/dark_mode_side_effect';
import { type EditorState } from '@codemirror/state';

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
export const createFlusterStore = (webviewEnv: WebviewEnvironment) => {
    const isProd = import.meta.env.FLUSTER_PROD_BUILD === "true";
    const listener = createListenerMiddleware({
        extra: {
            /**
             * The `WebviewEnvironment` used throughout the app injected into
             * global redux state. This will need to be accesse via the listener API 
             * I thnk...
             */
            webviewEnvironment: webviewEnv
        }
    })
    listener.startListening({
        actionCreator: setDarkMode,
        effect: (d, a) => {
            const newDarkMode = d.payload === "toggle" ? !(a.getState() as MdxEditorAppState).container.dark_mode : d.payload
            if (newDarkMode) {
                document.body.classList.add("dark")
            } else {
                document.body.classList.remove("dark")
            }
        }
    })
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
            }).prepend(listener.middleware).prepend(darkModeListenerMiddleware.middleware),
    });
    const persistor = persistStore(store);
    return { store, persistor };
};


export type MdxEditorStore = ReturnType<typeof createFlusterStore>["store"]
export type MdxEditorPersistor = ReturnType<typeof createFlusterStore>["persistor"]
export type AppDispatch = MdxEditorStore["dispatch"]

