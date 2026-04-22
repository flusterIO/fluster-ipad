(() => {

    function handleAdmonitionHeight(container: HTMLDivElement) {
        const bodyContainer = container.querySelector(".cdrm-admon-body-container") as HTMLDivElement;
        const body = container.querySelector(".cdrm-admon-body") as HTMLDivElement;
        body.style.height = 'auto';
        body.style.transition = 'max-height 500ms ease-in-out';
        const bodyHeight = body.getBoundingClientRect().height;
        bodyContainer.style.maxHeight = `${bodyHeight}px`
    }


    function openAdmonition(container: HTMLDivElement) {
        container.setAttribute("data-cdrm-folded", "false");
        handleAdmonitionHeight(container)
    }

    function closeAdmonition(container: HTMLDivElement) {
        const bodyContainer = container.querySelector(".cdrm-admon-body-container") as HTMLDivElement;
        bodyContainer.style.maxHeight = "0px";
        container.setAttribute("data-cdrm-folded", "true");
    }


    let ems = document.getElementsByClassName("cdrm-admon-title-group") as HTMLCollectionOf<HTMLDivElement>;
    for (var i = 0; i < ems.length; i++) {
        const item = ems.item(i);
        item.addEventListener("click", (e) => {
            const container = (e.currentTarget as HTMLDivElement).parentElement as HTMLDivElement;
            let folded = container.getAttribute("data-cdrm-folded") === "true";
            let foldable = container.getAttribute("data-cdrm-foldable") === "true";
            const body = container.querySelector(".cdrm-admon-body-container");
            if (!body) {
                return
            }
            if (foldable) {
                if (folded) {
                    openAdmonition(container)
                } else {
                    closeAdmonition(container)
                }
            }
        })
    }

    function handleAdmonitionResize() {
        const ems = document.getElementsByClassName("cdrm-admon");
        for (var i = 0; i < ems.length; i++) {
            const item = ems.item(i) as HTMLDivElement;
            if (item.getAttribute("data-cdrm-folded") === "false" && item.getAttribute("data-cdrm-foldable") === "true") {
                handleAdmonitionHeight(item)
            }
        }
    }

    window.addEventListener("resize", handleAdmonitionResize);
    window.addEventListener("cdrm-manual-resize", handleAdmonitionResize);
})()
