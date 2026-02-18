import { ViewPlugin } from "@codemirror/view";

declare global {
    interface WindowEventMap {
        "set-editor-scroll-proportion": CustomEvent<number>;
        "request-editor-scroll-proportion": CustomEvent<null>;
    }
}


export const sendEditorScrollDOMEvent = (d: HTMLElement) => {
    const { scrollTop, scrollHeight, clientHeight } = d
    const maxScroll = scrollHeight - clientHeight;
    const proportion = maxScroll > 0 ? scrollTop / maxScroll : 0;

    window.dispatchEvent(new CustomEvent("set-editor-scroll-proportion", {
        detail: proportion
    }))
}

export const scrollPlugin = ViewPlugin.define((view) => {
    return {
        scroll: () => {
            sendEditorScrollDOMEvent(view.scrollDOM)
        }
    };
}, {
    eventHandlers: {
        scroll: function () {
            this.scroll();
        }
    }
});

