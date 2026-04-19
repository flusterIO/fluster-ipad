(() => {
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
})()


const copyCodeblockCode = (e: Event) => {
    console.log(`Target: ${e}`)
}
