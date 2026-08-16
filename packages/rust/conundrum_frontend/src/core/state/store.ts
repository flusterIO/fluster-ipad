import {
    type Reducer,
    combineReducers,
    configureStore,
} from "@reduxjs/toolkit";
import { type AppState } from "./initial_state";
import NavigationReducer from "../../features/navigation/state/navigation_slice";
import SearchReducer from "../../features/search/state/search_slice";
import DatabaseReducer from "../../features/database/state/database_slice";
import UIReducer from "../../features/ui/state/ui_state_slice";
import AIReducer from "../../features/ai/state/ai_state_slice";
import NotificationReducer from "../../features/notifications/state/notification_state_slice";
import {
    persistReducer,
    type PersistConfig,
    FLUSH,
    REHYDRATE,
    PAUSE,
    PERSIST,
    PURGE,
    REGISTER,
} from "redux-persist";
import autoMergeLevel2 from "redux-persist/lib/stateReconciler/autoMergeLevel2";
import indexedDb from "redux-persist-indexeddb-storage";

const reducers: Record<keyof AppState, Reducer> = {
    search: SearchReducer,
    navigation: NavigationReducer,
    database: DatabaseReducer,
    ui: UIReducer,
    notification: NotificationReducer,
    ai: AIReducer,
};

const rootReducer = combineReducers(reducers);

const persistConfig: PersistConfig<AppState> = {
    key: "root",
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-call
    storage: indexedDb("conundrum"),
    blacklist: [],
    stateReconciler: autoMergeLevel2,
};

const persistedReducer = persistReducer(persistConfig, rootReducer);

const store = configureStore({
    reducer: persistedReducer,
    middleware: (getDefaultMiddleware) =>
        getDefaultMiddleware({
            serializableCheck: {
                ignoredActions: [FLUSH, REHYDRATE, PAUSE, PERSIST, PURGE, REGISTER],
            },
        }),
});

export default store;
