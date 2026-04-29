import consola from "consola";
import flusterWasm from "@fluster/wasm"
import { initializeConundrumWeb } from "@conundrum/ts";
declare global {

    interface WindowEventMap {
        "wasm-loaded": CustomEvent;
    }
}

export const iniitializeWebView = () => {


    void flusterWasm().then(() => {
        consola.success("Wasm loaded.");
        window.dispatchEvent(new CustomEvent("wasm-loaded"))
    });
    initializeConundrumWeb();
}
