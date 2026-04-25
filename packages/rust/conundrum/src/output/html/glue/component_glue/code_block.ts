(() => {
    function copyCodeblockCode(e: Event) {
        let target = e.currentTarget as HTMLDivElement;
        console.log("target: ", target)
        const targetId = target.getAttribute("data-cdrm-copy-for");
        console.log("targetId: ", targetId)
        const parentEm = document.getElementById(targetId);
        console.log("parentEm: ", parentEm)
        window.navigator.clipboard.writeText(parentEm.querySelector("pre")?.innerText ?? "")
        window.dispatchEvent(new CustomEvent("cdrm-codeblock-copy"))
    }
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
        item.addEventListener("click", copyCodeblockCode)
    }
})()
