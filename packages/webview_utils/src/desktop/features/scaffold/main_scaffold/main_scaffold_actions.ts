import { prefersDarkMode } from "src/desktop/core/ui_utils/prefers_dark_mode";
import { ThemeMode } from "./state/main_scaffold_initial_state";
import { AppState } from "src/desktop/core/state/initial_state";
import store from "src/desktop/core/state/store";
import { setThemeAction, setThemeModeAction } from "./state/main_scaffold_slice";
import { supportedTheme } from "src/desktop/core/core_types/supported_themes";

export const toggleDarkMode = () => {
    const currentThemeMode = (store.getState() as AppState).scaffold.themeMode;
    const themeMode =
        currentThemeMode === ThemeMode.system
            ? prefersDarkMode()
                ? ThemeMode.light
                : ThemeMode.dark
            : currentThemeMode === ThemeMode.light
                ? ThemeMode.dark
                : ThemeMode.light;
    document.body.classList[themeMode === ThemeMode.dark ? "add" : "remove"](
        "dark"
    );
    store.dispatch(setThemeModeAction(themeMode));
};



export const setTheme = (newTheme: (typeof supportedTheme)[number]) => {
    document.body.setAttribute("data-fluster-theme", newTheme);
    store.dispatch(setThemeAction(newTheme));
};
