import { ToastConfig } from "@fluster/desktop_bindings";
import { v4 as uuidv4 } from "uuid";

export const showToast = (data: Omit<ToastConfig, "id">) => {
    window.dispatchEvent(
        new CustomEvent("show-toast", {
            detail: {
                ...data,
                id: uuidv4(),
            },
        })
    );
    // return ;
};

export const showErrorToast = (body: string, title = "Something went wrong") =>
    showToast({
        body,
        title,
        variant: "Error",
        duration: 5000,
    });
