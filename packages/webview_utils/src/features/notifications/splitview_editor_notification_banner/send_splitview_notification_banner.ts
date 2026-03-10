import { v4 } from "uuid";
import { useDispatch } from "react-redux";
import { appendBannerNotifcation } from "#/webview_global_state/notification_state/notification_state_slice";
import { type EditorBannerNotification } from "@/code_gen/typeshare/fluster_core_utilities";

export const useSendNotificationBanner = () => {
    const dispatch = useDispatch()

    return (item: Omit<EditorBannerNotification, "id">) => {
        dispatch(appendBannerNotifcation({
            ...item,
            id: v4()
        }))
    }
}
