export enum AppLoadingState {
    AppLoading,
    Syncing,
}

export interface ScaffoldState {
    loading: AppLoadingState[];
}
