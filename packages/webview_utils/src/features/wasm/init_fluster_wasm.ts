import { greet } from "@fluster/wasm"


declare global {

    interface Window {
        "greet": typeof greet
    }
}

window.greet = greet

export {
    greet
}
