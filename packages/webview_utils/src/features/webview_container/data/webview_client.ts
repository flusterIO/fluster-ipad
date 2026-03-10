import { type MdxEditorAppState } from "../../webview_global_state/store"

export const WebviewClient = {
    sendPanelGroupResize: (layout: number[]) => {
        window.dispatchEvent(new CustomEvent("main-panel-resize", {
            detail: layout
        }))
    },
    setMdxContentLoaded: (contentLoadedId: string) => {
        window.dispatchEvent(new CustomEvent("mdx-content-loaded", {
            detail: {
                scrollPositionKey: contentLoadedId
            }
        }))
    },
    setDarkMode: (isDark: boolean) => {
        if (isDark) {
            document.body.classList.add('dark');
        } else {
            document.body.classList.remove('dark');
        }
    },
    applyGlobalState: (state: MdxEditorAppState) => {
        WebviewClient.setDarkMode(state.container.dark_mode)
    }
}

