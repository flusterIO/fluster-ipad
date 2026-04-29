

export const applyCopyConundrumCodeBlockListeners = (): void => {
    const ems = document.getElementsByClassName("cdrm-codeblock");
    for (var i = 0; i < ems.length; i++) {
        const item: HTMLDivElement = ems.item(i) as HTMLDivElement;
        item.addEventListener("mouseenter", (e) => {
            (e.target as HTMLDivElement).classList.add("hovered")
        })
        item.addEventListener("mouseleave", (e) => {
            (e.target as HTMLDivElement).classList.add("hovered")
        })
    }
    const icons = document.getElementsByClassName("cdrm-codeblock-icon");
    for (var i = 0; i < icons.length; i++) {
        const item = icons.item(i) as HTMLDivElement;
        item.addEventListener("click", async (e) => {
            import("@conundrum/ts/methods").then((a) => a.onCopyCodeBlockClick(e));
        })
    }
}

(() => {
    applyCopyConundrumCodeBlockListeners();
    window.addEventListener("cdrm-content-loaded", applyCopyConundrumCodeBlockListeners)
})();
