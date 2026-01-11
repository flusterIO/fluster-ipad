import { SettingsState } from "./settings_state";

export const initialSettingsState: SettingsState = {
    notesDirectory: "",
    backupDirectory: null,
    hasLoadedSavedState: false,
    nThreads: 8,
    useGitIgnore: false,
    dashboardType: "dashboard",
};


