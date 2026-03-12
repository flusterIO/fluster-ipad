import { createListenerMiddleware } from '@reduxjs/toolkit';
import { type GlobalAppState } from '#/webview_global_state/store';
import { sendToSwift } from '@/utils/bridge/send_to_swift';
import { type EditorChangeEvent, SplitviewEditorWebviewActions } from '@/code_gen/typeshare/fluster_core_utilities';

export const editorValueChangeListenerMiddleware = createListenerMiddleware<GlobalAppState>();

editorValueChangeListenerMiddleware.startListening({
    predicate(_, currentState, originalState) {
        return currentState.editor.value !== originalState.editor.value
    },
    effect: (_, listenerApi) => {
        const state = listenerApi.getState()
        if (!state.editor.note_id) {
            return
        }
        sendToSwift(SplitviewEditorWebviewActions.OnEditorChange, JSON.stringify({
            note_id: state.editor.note_id,
            content: state.editor.value
        } satisfies EditorChangeEvent))
    },
});


