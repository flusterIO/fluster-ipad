import { type GlobalAppState } from "#/webview_global_state/store"
import { useSelector } from "react-redux"

export const useWebviewEnvironment = () => {
    const environment = useSelector((state: GlobalAppState) => state.container.environment)
    return environment
}
