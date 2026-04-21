(() => {
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
                    container.setAttribute("data-cdrm-folded", "false")
                } else {
                    container.setAttribute("data-cdrm-folded", "true")
                }
            }
        })
    }
})()
