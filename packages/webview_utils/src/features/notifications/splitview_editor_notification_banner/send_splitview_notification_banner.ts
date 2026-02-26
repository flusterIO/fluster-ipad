import { EditorNotificationBanner } from "./types";
import { v4 } from "uuid";

declare global {
    interface WindowEventMap {
        "show-editor-banner": CustomEvent<EditorNotificationBanner>;
    }
}

export const sendSplitviewNotificationBanner = (item: Omit<EditorNotificationBanner, "id">) => {
    window.dispatchEvent(new CustomEvent("show-editor-banner", {
        detail: {
            ...item,
            id: v4()
        }
    }))
}
