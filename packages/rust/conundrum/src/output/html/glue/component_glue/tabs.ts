import { onTabResize } from "@conundrum/ts/methods"



(() => {
    function addConundrumTabClickListeners() {
        function handleTabClick(e: Event) {
            const em = (e.currentTarget as HTMLDivElement).parentElement!.parentElement as HTMLDivElement;
            if (!em) {
                return
            }
            const emphasis = em.getAttribute("data-cdrm-emphasis");
            let tabs = em.querySelectorAll(".cdrm-tab-subtle-border");
            const clickedIndex = parseInt((e.currentTarget as HTMLDivElement).getAttribute("data-cdrm-idx") as string)
            const groupId = em.getAttribute("data-cdrm-group");
            const lastFocusedIndex = parseInt(em.getAttribute("data-cdrm-focused-idx") as string)
            for (var i = 0; i < tabs.length; i++) {
                const tab = tabs.item(i) as HTMLButtonElement;
                let bgClasses = tab.classList.values().toArray().filter((s) => s.startsWith("bg-"));
                for (const k of bgClasses) {
                    tab.classList.remove(k);
                }
                if (i === clickedIndex) {
                    let activeTabBorder = (e.currentTarget as HTMLDivElement).querySelector(".cdrm-tab-subtle-border") as HTMLDivElement;

                    if (activeTabBorder) {
                        activeTabBorder.style.transformOrigin = lastFocusedIndex < clickedIndex ? "left" : "right";
                        activeTabBorder.classList.remove("bg-transparent");
                        activeTabBorder.classList.remove("scale-x-0");
                        // DO NOT SCAN THESE FILES WITH TAILWIND OR SHIT WILL EXPLODE
                        activeTabBorder.classList.add(`bg-emphasis-${emphasis}`)
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
                    if (i === clickedIndex) {
                        tabBody.style.opacity = "1"
                    } else {
                        tabBody.style.opacity = "0"
                    }
                }
            }
        }
        const ems = document.getElementsByClassName("cdrm-tab-btn");
        for (var i = 0; i < ems.length; i++) {
            const item = ems.item(i) as HTMLButtonElement;
            item.addEventListener("click", handleTabClick)
        }
    }
    addConundrumTabClickListeners();
    window.addEventListener("resize", onTabResize);
    window.addEventListener("cdrm-content-loaded", addConundrumTabClickListeners);
})();
