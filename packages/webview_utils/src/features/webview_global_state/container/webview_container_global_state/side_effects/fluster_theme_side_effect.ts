
import { createListenerMiddleware } from '@reduxjs/toolkit';
import { type MdxEditorAppState } from '#/webview_global_state/store';

export const flusterThemeListenerMiddleware = createListenerMiddleware<MdxEditorAppState>();

flusterThemeListenerMiddleware.startListening({
    predicate(_, currentState, originalState) {
        return currentState.container.fluster_theme !== originalState.container.fluster_theme
    },
    effect: (_, listenerApi) => {
        const theme = listenerApi
            .getState()
            .container.fluster_theme
        document.body.setAttribute("data-fluster-theme", theme);
    },
});

