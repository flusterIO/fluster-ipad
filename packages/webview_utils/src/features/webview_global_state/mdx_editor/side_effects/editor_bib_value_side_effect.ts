
import { createListenerMiddleware } from '@reduxjs/toolkit';
import { type GlobalAppState } from '#/webview_global_state/store';
import { sendToSwift } from '@/utils/bridge/send_to_swift';
import { SplitviewEditorWebviewActions } from '@/code_gen/typeshare/fluster_core_utilities';

export const editorBibValueChangeListenerMiddleware = createListenerMiddleware<GlobalAppState>();

editorBibValueChangeListenerMiddleware.startListening({
    predicate(_, currentState, originalState) {
        return currentState.editor.bib_editor.value !== originalState.editor.bib_editor.value
    },
    effect: (_, listenerApi) => {
        const state = listenerApi.getState()
        if (!state.editor.note_id) {
            return
        }
        sendToSwift(SplitviewEditorWebviewActions.OnBibEditorChange, state.editor.bib_editor.value)
    },
});


