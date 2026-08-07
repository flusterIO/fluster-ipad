import { databaseInitialState as initialDatabaseState } from "#/database/state/database_initial_state";
import { type DatabaseState } from "#/database/state/database_state";
import { initialNavigationState } from "#/navigation/state/initial_navigation_state";
import { type NavigationState } from "#/navigation/state/navigation_state";
import { initialSearchState } from "#/search/state/initial_search_state";
import { type SearchState } from "#/search/state/search_state";
import { type UIState } from "#/ui/state/ui_state";
import { initialUIState } from "#/ui/state/initial_ui_state";
import { type NotificationState } from "#/notifications/state/notification_state";
import { initialNotificationState } from "#/notifications/state/initial_notification_state";

export interface AppState {
    search: SearchState;
    navigation: NavigationState;
    database: DatabaseState;
    ui: UIState;
    notification: NotificationState;
}

export const initialState: AppState = {
    search: initialSearchState,
    navigation: initialNavigationState,
    database: initialDatabaseState,
    ui: initialUIState,
    notification: initialNotificationState,
};
