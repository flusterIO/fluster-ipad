import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { AnyWebviewAction } from "@/utils/types/any_window_event";
import { useEffect } from "react";

export const useWebviewLoadedEvent = (action: AnyWebviewAction) => {

    useEffect(() => {
        sendToSwift(action)
    }, [action]);

    return null
}
