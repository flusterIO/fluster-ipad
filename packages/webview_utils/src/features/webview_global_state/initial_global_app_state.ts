import { type GlobalWebviewState } from "@/code_gen/typeshare/fluster_core_utilities";
import { initialAiState } from "./ai_state/initial_ai_state";
import { initialWebviewContainerState } from "./container/webview_container_global_state/initial_webview_container_state";
import { initialEditorState } from "./mdx_editor/initial_editor_state";
import { initialMediaState } from "./media_state/initial_media_state";
import { initialNotificationState } from "./notification_state/initial_notification_state";

export const initialGlobalAppState: GlobalWebviewState = {
    ai: initialAiState,
    container: initialWebviewContainerState,
    editor: initialEditorState,
    media: initialMediaState,
    notifications: initialNotificationState,
}
