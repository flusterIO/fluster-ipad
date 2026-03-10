import { type MdxEditorAppState } from "#/webview_global_state/store"
import { useSelector } from "react-redux"

export const useWebviewEnvironment = () => {
    const environment = useSelector((state: MdxEditorAppState) => state.container.environment)
    return environment
}
