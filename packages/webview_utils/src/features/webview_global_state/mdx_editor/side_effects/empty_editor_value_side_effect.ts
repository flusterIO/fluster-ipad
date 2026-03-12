import { type AnyListenerPredicate, createListenerMiddleware } from '@reduxjs/toolkit';
import { type GlobalAppState } from '#/webview_global_state/store';
import { sendToSwift } from '@/utils/bridge/send_to_swift';
import { SplitviewEditorWebviewActions } from '@/code_gen/typeshare/fluster_core_utilities';

export const emptyValueListenerMiddleware = createListenerMiddleware<GlobalAppState>();

const emptyValuePredicate: AnyListenerPredicate<GlobalAppState> = (_, state) => {
    return typeof state.editor.value !== "string"
}

emptyValueListenerMiddleware.startListening({
    predicate: emptyValuePredicate,
    effect: () => {
        sendToSwift(SplitviewEditorWebviewActions.RequestSplitviewEditorData);
    },
});

