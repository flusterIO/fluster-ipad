import { initialAiState } from "./ai_state/initial_ai_state";
import { initialWebviewContainerState } from "./container/webview_container_global_state/initial_webview_container_state";
import { initialEditorState } from "./mdx_editor/initial_editor_state";
import { initialMediaState } from "./media_state/initial_media_state";
import { initialNotificationState } from "./notification_state/initial_notification_state";
import { type GlobalWebviewStateDeepNullable, type GlobalWebviewStateNullable } from "./cross_language_state_types";
import { initialDictionaryState } from "./dictionary_state/initial_dictionary_state";

export const initialGlobalAppState: GlobalWebviewStateDeepNullable = {
    ai: initialAiState,
    container: initialWebviewContainerState,
    editor: initialEditorState,
    media: initialMediaState,
    notifications: initialNotificationState,
    note_details: null,
    dictionary: initialDictionaryState
}
