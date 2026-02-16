import { useWebviewContainerContext } from "../webview_provider"

export const useWebviewEnvironment = () => {
    const { environment } = useWebviewContainerContext()
    return environment
}
