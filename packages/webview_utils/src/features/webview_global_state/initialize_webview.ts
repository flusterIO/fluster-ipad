import consola from "consola";
import cdrmWasm from "@conundrum/wasm"
import { initializeConundrumWeb } from "@conundrum/ts";
declare global {
    interface WindowEventMap {
        "wasm-loaded": CustomEvent;
    }
}

export const initializeWebView = () => {
    initializeConundrumWeb();
    cdrmWasm().then(() => {
        consola.info("Conundrum WASM loaded")
    })
        .catch((err: unknown) => {
            consola.error("Error: ", err)
        })
}
