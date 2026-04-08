import consola from "consola";

export const iniitializeWebView = () => {
    /* eslint-disable-next-line  -- Being extra safe just incase I'm early. Safety first... */
    window.MathJax?.Hub?.Register?.StartupHook("typesetAll", () => {
        const elements = document.getElementsByClassName("conundrum-math");
        console.log("elements: ", elements)
        window.MathJax.Hub.Typeset(elements, () => {
            consola.success("Typeset math successfully.")
        })
    })
}
