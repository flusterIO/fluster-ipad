import SettingsReducer from "../../features/settings/state/settings_state_slice";
import PanelLeftReducer from "../../features/panel_left/state/panel_left_slice";
import PanelRightReducer from "../../features/panel_right/state/panel_right_slice";
import ScaffoldReducer from "../../features/scaffold/state/scaffold_state_slice";
import CommandPaletteReducer from "../../features/command_palette/state/command_palette_state_slice";
import KeymapReducer from "../../features/keymap/state/keymap_state_slice";
import NotificationsReducer from "../../features/notifications/state/notification_state_slice";
import { Reducer, combineReducers, configureStore } from "@reduxjs/toolkit";
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
import autoMergeLevel2 from "redux-persist/lib/stateReconciler/autoMergeLevel2";
import { stateStorage } from "./state_storage";
import { AppState } from "./initial_state";

const reducers: Record<keyof AppState, Reducer> = {
    panelLeft: PanelLeftReducer,
    panelRight: PanelRightReducer,
    settings: SettingsReducer,
    scaffold: ScaffoldReducer,
    commandPalette: CommandPaletteReducer,
    keymap: KeymapReducer,
    notifications: NotificationsReducer,
};

const rootReducer = combineReducers(reducers);

const persistConfig: PersistConfig<AppState> = {
    key: "root",
    storage: stateStorage,
    blacklist: ["settings.syncing"],
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
