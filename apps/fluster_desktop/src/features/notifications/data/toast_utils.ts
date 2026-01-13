import store from "@/state/store";
import { ToastConfig } from "@fluster/desktop_bindings";
import { appendNotification } from "../state/notification_state_slice";

export type ToastConfigTemplate = Omit<
    ToastConfig,
    "id" | "duration" | "variant"
> & {
    duration?: ToastConfig["duration"];
    variant?: ToastConfig["variant"];
};

export const showToast = (data: ToastConfigTemplate) => {
    store.dispatch(
        appendNotification({
            ...data,
            duration: data.duration ?? 5000,
            variant: data.variant ?? "Info",
        }),
    );
};
