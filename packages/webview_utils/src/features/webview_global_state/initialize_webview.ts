import { initializeConundrumWeb, attachConundrumWasm } from "@conundrum/ts";
declare global {
    interface WindowEventMap {
        "wasm-loaded": CustomEvent;
    }
}

export const initializeWebView = () => {
    initializeConundrumWeb();
    attachConundrumWasm();
}
