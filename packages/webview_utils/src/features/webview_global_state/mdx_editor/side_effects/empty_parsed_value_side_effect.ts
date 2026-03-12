
import { type AnyListenerPredicate, createListenerMiddleware } from '@reduxjs/toolkit';
import { type GlobalAppState } from '#/webview_global_state/store';
import { sendToSwift } from '@/utils/bridge/send_to_swift';
import { SplitviewEditorWebviewActions } from '@/code_gen/typeshare/fluster_core_utilities';

export const emptyParsedValueListenerMiddleware = createListenerMiddleware<GlobalAppState>();

const emptyParsedValuePredicate: AnyListenerPredicate<GlobalAppState> = (_, state) => {
    return typeof state.editor.parsedValue !== "string"
}

emptyParsedValueListenerMiddleware.startListening({
    predicate: emptyParsedValuePredicate,
    effect: () => {
        sendToSwift(SplitviewEditorWebviewActions.RequestSplitviewEditorData);
    },
});

