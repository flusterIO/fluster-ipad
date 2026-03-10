import { createListenerMiddleware, isAnyOf } from '@reduxjs/toolkit';
import { setDarkMode } from '../webview_container_slice';
import { type MdxEditorAppState } from '#/webview_global_state/store';

export const darkModeListenerMiddleware = createListenerMiddleware();

darkModeListenerMiddleware.startListening({
    matcher: isAnyOf(setDarkMode),
    effect: (_, listenerApi) => {
        const isDark = (listenerApi.getState() as MdxEditorAppState).container.dark_mode;
        if (isDark) {
            document.body.classList.add('dark');
        } else {
            document.body.classList.remove('dark');
        }
    },
});

