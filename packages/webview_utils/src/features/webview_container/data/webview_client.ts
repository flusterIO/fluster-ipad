class WebviewClient {
    constructor() { }
    static sendPanelGroupResize(layout: number[]) {
        window.dispatchEvent(new CustomEvent("main-panel-resize", {
            detail: layout
        }))
    }
    static setMdxContentLoaded(contentLoadedId: string) {
        window.dispatchEvent(new CustomEvent("mdx-content-loaded", {
            detail: {
                scrollPositionKey: contentLoadedId
            }
        }))
    }
}


export {
    WebviewClient
}
