import { ToastConfig } from "@fluster/desktop_bindings";
import { ConfirmationModalProps } from "../features/notifications/confirmation/confirmation_types";

export interface ConfirmationResponse {
    /// True if the confirmation was accepted, false otherwise.
    response: boolean;
    /// The confirmation id.
    id: string;
}

declare global {

    interface WindowEventMap {
        "show-toast": CustomEvent<Omit<ToastConfig, "createdAt">>;
        "confirmation-response": CustomEvent<ConfirmationResponse>;
        "request-confirmation-response": CustomEvent<ConfirmationModalProps>;
        "cancel-confirmation-request": CustomEvent<{ id: string }>;
        "show_command_palette": CustomEvent<null>;
    }
}
