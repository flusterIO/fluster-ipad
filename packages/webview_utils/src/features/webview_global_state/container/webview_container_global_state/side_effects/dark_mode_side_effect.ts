import { createListenerMiddleware } from '@reduxjs/toolkit';
import { type MdxEditorAppState } from '#/webview_global_state/store';
import { WebviewClient } from '../../../../webview_container/data/webview_client';

export const darkModeListenerMiddleware = createListenerMiddleware<MdxEditorAppState>();

darkModeListenerMiddleware.startListening({
    predicate: (_, oldState, newState) => {
        return oldState.container.dark_mode !== newState.container.dark_mode
    },
    effect: (_, listenerApi) => {
        WebviewClient.setDarkMode(listenerApi.getState().container.dark_mode)
    },
});

