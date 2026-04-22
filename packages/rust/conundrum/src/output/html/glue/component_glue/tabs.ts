(() => {
    function handleTabClick(e: Event) {
        const em = (e.currentTarget as HTMLDivElement).parentElement.parentElement as HTMLDivElement;
        console.log("em: ", em)
        if (!em) {
            return
        }
        const emphasis = em.getAttribute("data-cdrm-emphasis");
        console.log("emphasis: ", emphasis)
        let tabs = em.querySelectorAll(".cdrm-tab-subtle-border");
        console.log("tabs: ", tabs)
        const clickedIndex = parseInt((e.currentTarget as HTMLDivElement).getAttribute("data-cdrm-idx"))
        const groupId = em.getAttribute("data-cdrm-group");
        console.log("clickedIndex: ", clickedIndex)
        const lastFocusedIndex = parseInt(em.getAttribute("data-cdrm-focused-idx"))
        for (var i = 0; i < tabs.length; i++) {
            const tab = tabs.item(i) as HTMLButtonElement;
            let bgClasses = tab.classList.values().toArray().filter((s) => s.startsWith("bg-"));
            for (const k of bgClasses) {
                tab.classList.remove(k);
            }
            if (i === clickedIndex) {
                let activeTabBorder = (e.currentTarget as HTMLDivElement).querySelector(".cdrm-tab-subtle-border") as HTMLDivElement;
                console.log("activeTabBorder: ", activeTabBorder)

                if (activeTabBorder) {
                    activeTabBorder.style.transformOrigin = lastFocusedIndex < clickedIndex ? "left" : "right";
                    activeTabBorder.classList.remove("bg-transparent");
                    activeTabBorder.classList.remove("scale-x-0");
                    // DO NOT SCAN THESE FILES WITH TAILWIND OR SHIT WILL EXPLODE
                    activeTabBorder.classList.add(`bg-${emphasis}`)
                }
            } else {
                tab.style.transformOrigin = lastFocusedIndex > clickedIndex ? "left" : "right";
                tab.classList.add("bg-transparent");
                tab.classList.add("scale-x-0");
            }
        }
        em.setAttribute("data-cdrm-focused-idx", `${clickedIndex}`)

        const allTabBodies = document.getElementsByClassName("cdrm-tab-group-item");

        for (var i = 0; i < allTabBodies.length; i++) {
            const tabBody = allTabBodies.item(i) as HTMLDivElement;
            if (tabBody.getAttribute("data-cdrm-group") === groupId) {
                tabBody.style.transform = `translateX(${(i - clickedIndex) * 100}%)`
            }
            // if (i === clickedIndex) {
            //     tabBody.classList.add("relative")
            //     tabBody.classList.remove("absolute")
            // } else {
            //     tabBody.classList.remove("relative")
            //     tabBody.classList.add("absolute")
            // }
        }
    }
    /// If the div passed in is not a valid container this will break.
    function handleHeight(container: HTMLDivElement) {
        const focusedIndex = parseInt(container.getAttribute("data-cdrm-focused-idx"));
        const groupId = container.getAttribute("data-cdrm-group");
        const focusedTabBody = container.querySelector(`#tab-${groupId}-${focusedIndex}`) as HTMLDivElement;
        if (focusedTabBody) {
            const h = focusedTabBody.getBoundingClientRect().height;
            const bodyWrapper = container.querySelector(`#tab-body-wrapper-${groupId}`) as HTMLDivElement;
            if (bodyWrapper) {
                bodyWrapper.style.transition = "height 0.3s ease-in-out";
                bodyWrapper.style.height = `${h}px`;
            }
        }
    }
    const ems = document.getElementsByClassName("cdrm-tab-btn");
    for (var i = 0; i < ems.length; i++) {
        const item = ems.item(i) as HTMLButtonElement;
        item.addEventListener("click", handleTabClick)
    }
    const gatheHeightOfAllTabGroups = (): void => {
        const containers = document.getElementsByClassName("cdrm-tab-group");
        for (var i = 0; i < containers.length; i++) {
            const tabGroup = containers.item(i) as HTMLDivElement;

            const observer = new MutationObserver(() => {
                handleHeight(tabGroup)
            })
            observer.observe(tabGroup, {
                attributes: true,
                attributeFilter: ["data-cdrm-focused-idx"]
            })
            handleHeight(tabGroup)
        }
    }
    gatheHeightOfAllTabGroups()
    window.addEventListener("resize", gatheHeightOfAllTabGroups)
})();
