declare global {

    interface WindowEventMap {
        "main-panel-resize": CustomEvent<number[]>;
    }
}

export { }
