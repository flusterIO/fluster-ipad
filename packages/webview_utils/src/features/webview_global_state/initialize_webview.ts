import consola from "consola";
import flusterWasm from "@fluster/wasm"
declare global {

    interface WindowEventMap {
        "wasm-loaded": CustomEvent;
    }
}

export const iniitializeWebView = () => {
    /* eslint-disable-next-line  -- Being extra safe just incase I'm early. Safety first... */
    window.MathJax?.Hub?.Register?.StartupHook("typesetAll", () => {
        const elements = document.getElementsByClassName("conundrum-math");
        window.MathJax.Hub.Typeset(elements, () => {
            consola.success("Typeset math successfully.")
        })
    })


    void flusterWasm().then(() => {
        consola.success("Wasm loaded.");
        window.dispatchEvent(new CustomEvent("wasm-loaded"))
    });
}
