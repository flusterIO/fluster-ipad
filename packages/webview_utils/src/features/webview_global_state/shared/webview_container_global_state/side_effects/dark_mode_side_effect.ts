import { createListenerMiddleware, isAnyOf } from '@reduxjs/toolkit';
import { setDarkMode } from '../webview_container_slice';
import { MdxEditorAppState } from '#/webview_global_state/mdx_editor/store';

export const listenerMiddleware = createListenerMiddleware();

listenerMiddleware.startListening({
    matcher: isAnyOf(setDarkMode),
    effect: async (_, listenerApi) => {
        const isDark = (listenerApi.getState() as MdxEditorAppState).container.dark_mode;
        if (isDark) {
            document.body.classList.add('dark');
        } else {
            document.body.classList.remove('dark');
        }
    },
});

