import { databaseInitialState as initialDatabaseState } from "#/database/state/database_initial_state";
import { type DatabaseState } from "#/database/state/database_state";
import { initialNavigationState } from "#/navigation/state/initial_navigation_state";
import { type NavigationState } from "#/navigation/state/navigation_state";
import { initialSearchState } from "#/search/state/initial_search_state";
import { type SearchState } from "#/search/state/search_state";

export interface AppState {
    search: SearchState;
    navigation: NavigationState;
    database: DatabaseState;
}

export const initialState: AppState = {
    search: initialSearchState,
    navigation: initialNavigationState,
    database: initialDatabaseState,
};
