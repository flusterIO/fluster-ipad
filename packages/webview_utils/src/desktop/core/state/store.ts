import { Reducer, combineReducers, configureStore } from "@reduxjs/toolkit";
import ScaffoldReducer from "../../features/scaffold/main_scaffold/state/main_scaffold_slice";
import SettingsReducer from "../../features/settings/state/settings_slice"
import BibliographyReducer from "../../features/bibliography/state/bib_state_slice"
import CodeReducer from "../../features/code/state/code_state_slice"
import PanelRightReducer from "../../features/panel_right/state/panel_right_state_slice"
import PanelLeftReducer from "../../features/panel_left/state/panel_left_state_slice"
import { AppState } from "./initial_state";
import {
    persistReducer,
    PersistConfig,
    FLUSH,
    REHYDRATE,
    PAUSE,
    PERSIST,
    PURGE,
    REGISTER,
} from "redux-persist";
import { stateStorage } from "./state_storage";
import autoMergeLevel2 from "redux-persist/lib/stateReconciler/autoMergeLevel2";

const reducers: Record<keyof AppState, Reducer> = {
    scaffold: ScaffoldReducer,
    settings: SettingsReducer,
    bib: BibliographyReducer,
    code: CodeReducer,
    panelRight: PanelRightReducer,
    panelLeft: PanelLeftReducer
};

const rootReducer = combineReducers(reducers);

const persistConfig: PersistConfig<AppState> = {
    key: "root",
    storage: stateStorage,
    blacklist: ["core.syncing"],
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
