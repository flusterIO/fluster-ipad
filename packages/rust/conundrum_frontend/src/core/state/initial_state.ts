import { initialNavigationState } from "#/navigation/state/initial_navigation_state";
import { type NavigationState } from "#/navigation/state/navigation_state";
import { initialSearchState } from "#/search/state/initial_search_state";
import { type SearchState } from "#/search/state/search_state";

export interface AppState {
    search: SearchState;
    navigation: NavigationState;
}

export const initialState: AppState = {
    search: initialSearchState,
    navigation: initialNavigationState,
};
