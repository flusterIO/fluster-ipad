import { type EcosystemLogInput } from "#/database/db_utility_types/log_types";
import { ecosystemLogToNotification } from "#/notifications/models/notifcation_item";
import { appendNotification } from "#/notifications/state/notification_state_slice";
import { rspc } from "@/app/rspc_client";
import consola from "consola";
import { useDispatch } from "react-redux";

export const useLogger = () => {
    const { mutateAsync } = rspc.useMutation("log.create", {
        onError(error) {
            consola.error("Failed to save log: ", error);
        },
    });
    const dispatch = useDispatch();
    const sendLog = async (
        log: EcosystemLogInput,
        broadcast = false,
    ): Promise<void> => {
        if (broadcast) {
            dispatch(appendNotification(ecosystemLogToNotification(log)));
        }
        try {
            await mutateAsync(log);
        } catch (err: unknown) {
            consola.error("Error: ", err);
        }
    };

    return sendLog;
};
