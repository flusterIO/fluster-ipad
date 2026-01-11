import { commands } from "@fluster/desktop_bindings";
import { AppState } from "src/desktop/core/state/initial_state";
import store from "src/desktop/core/state/store";
import { showToast } from "src/desktop/features/notifications/toasts/actions/show_toast";


export const fsFileExtensionGlob = async (
    fileExtensions: string
): Promise<string[]> => {
    const state: AppState = store.getState();
    if (!state.settings.notesDirectory.length) {
        showToast({
            title: "Oh no",
            body: "Cannot get pdf's. You have not yet set your note's directory.",
            variant: "Error",
            duration: 5000,
        });
        return [];
    }
    const res = await commands.fsFileExtensionGlob(
        fileExtensions,
        state.settings.notesDirectory,
        state.settings.nThreads?.toString()
    );
    if (res.status === "ok") {
        return res.data;
    } else {
        console.error(
            "An error occurred while performing a glob search: ",
            res.error
        );
        return [];
    }
};
