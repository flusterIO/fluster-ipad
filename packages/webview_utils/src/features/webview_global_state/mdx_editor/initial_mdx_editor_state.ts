import { initialWebviewContainerState } from "../shared/webview_container_global_state/initial_webview_container_state";
import { initialEditorState } from "./initial_editor_state";
import { type MdxEditorAppState } from "./store";

export const initialMdxEditorState: MdxEditorAppState = {
    editor: initialEditorState,
    container: initialWebviewContainerState
}
