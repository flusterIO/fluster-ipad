import { type GlobalAppState } from "../../webview_global_state/store"

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
        console.log("isDark: ", isDark)
        if (isDark) {
            document.body.classList.add('dark');
        } else {
            document.body.classList.remove('dark');
        }
    },
    applyGlobalState: (state: GlobalAppState) => {
        WebviewClient.setDarkMode(state.container.dark_mode)
    }
}

