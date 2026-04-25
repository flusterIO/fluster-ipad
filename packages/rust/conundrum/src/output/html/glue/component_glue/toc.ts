(() => {
    function handleTocHeadingClick(e: MouseEvent) {
        const target = e.currentTarget as HTMLDivElement;
        console.log("target: ", target)
        const heading = document.getElementById(`h-${target.getAttribute("cdrm-heading-id")}`)
        console.log("heading: ", heading)
        if (heading) {
            heading.scrollIntoView({
                behavior: "smooth"
            })
        }

    }
    const items = document.getElementsByClassName("cdrm-toc-item");
    console.log("items: ", items)
    for (var i = 0; i < items.length; i++) {
        const item = items.item(i);
        item.addEventListener("click", handleTocHeadingClick);
    }
})()
