import { type KeyboardShortcutItem } from "#/database/db_utility_types/keyboard";

export const executeKeyboardAction = (
    action: KeyboardShortcutItem["action"],
) => {
    console.log("kb: ", action);
};
