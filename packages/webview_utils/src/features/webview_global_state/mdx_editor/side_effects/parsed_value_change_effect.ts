import { type AnyListenerPredicate, createListenerMiddleware } from '@reduxjs/toolkit';
import { type GlobalAppState } from '#/webview_global_state/store';
import { SplitviewEditorDomIds } from '@/code_gen/typeshare/fluster_core_utilities';
import consola from 'consola';
import { ConundrumWebEvents } from '@/code_gen/typeshare/conundrum';

/// TODO: Move this to a separate Conundrum crate to mae it available to other.
declare global {
    interface WindowEventMap {
        [ConundrumWebEvents.CdrmContentLoaded]: CustomEvent;
        [ConundrumWebEvents.ManualResize]: CustomEvent;
    }
}

export const parsedValueChangeEffect = createListenerMiddleware<GlobalAppState>();

const parsedValueChangePredicate: AnyListenerPredicate<GlobalAppState> = (_, state, oldState) => {
    return state.editor.parsedValue !== oldState.editor.parsedValue
}

parsedValueChangeEffect.startListening({
    predicate: parsedValueChangePredicate,
    effect: (_, api) => {
        window.dispatchEvent(new CustomEvent(ConundrumWebEvents.CdrmContentLoaded))
        const container = document.getElementById(SplitviewEditorDomIds.MdxPreview) as HTMLDivElement | undefined | null;
        if (container) {
            const content = api.getState().editor.parsedValue;
            if (content) {
                container.innerHTML = content;
            }
        } else {
            consola.error("Fluster attempted to set Conundrum content before the container was ready.")
        }
    },
});

