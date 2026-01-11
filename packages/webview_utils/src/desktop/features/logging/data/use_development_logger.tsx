import { useEventListener } from "@/state/hooks/use_event_listener";
import store from "src/desktop/core/state/store";

declare global {
    interface WindowEventMap {
        "log-state": CustomEvent<object>;
    }
}

export const useDevelopmentLogger = (): null => {
    useEventListener("log-state", () => {
        const state = store.getState();
        console.log(state);
    });
    return null;
};
