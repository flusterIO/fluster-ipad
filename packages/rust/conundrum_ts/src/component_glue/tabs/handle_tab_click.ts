export const handleConundrumTabClick = (e: Event) => {
    const em = (e.currentTarget as HTMLDivElement | undefined)?.parentElement
        ?.parentElement as HTMLDivElement | undefined;
    if (!em) {
        return;
    }
    const emphasis = em.getAttribute("data-cdrm-emphasis");
    const tabs = em.querySelectorAll(".cdrm-tab-subtle-border");

    const ti = em.getAttribute("data-cdrm-idx");
    if (!ti) {
        return;
    }
    const clickedIndex = parseInt(ti);
    const groupId = em.getAttribute("data-cdrm-group");
    const focusedIdx = em.getAttribute("data-cdrm-focused-idx");
    if (typeof focusedIdx === "undefined") {
        return;
    }
    /* eslint-disable-next-line  -- I just checkedd.... I'm starting to hate eslint. */
    const lastFocusedIndex = parseInt(focusedIdx!);
    for (let i = 0; i < tabs.length; i++) {
        const tab = tabs.item(i) as HTMLButtonElement;
        const bgClasses = tab.classList
            .values()
            .toArray()
            .filter((s) => s.startsWith("bg-"));
        for (const k of bgClasses) {
            tab.classList.remove(k);
        }
        if (i === clickedIndex) {
            const activeTabBorder = em.querySelector(".cdrm-tab-subtle-border") as
                | HTMLDivElement
                | undefined;

            if (activeTabBorder) {
                activeTabBorder.style.transformOrigin =
                    lastFocusedIndex < clickedIndex ? "left" : "right";
                activeTabBorder.classList.remove("bg-transparent");
                activeTabBorder.classList.remove("scale-x-0");
                if (emphasis) {
                    // DO NOT SCAN THESE FILES WITH TAILWIND OR SHIT WILL EXPLODE
                    activeTabBorder.classList.add(`bg-emphasis-${emphasis}`);
                }
            }
        } else {
            tab.style.transformOrigin =
                lastFocusedIndex > clickedIndex ? "left" : "right";
            tab.classList.add("bg-transparent");
            tab.classList.add("scale-x-0");
        }
    }
    em.setAttribute("data-cdrm-focused-idx", `${clickedIndex}`);

    const allTabBodies = document.getElementsByClassName("cdrm-tab-group-item");

    for (let i = 0; i < allTabBodies.length; i++) {
        const tabBody = allTabBodies.item(i) as HTMLDivElement;
        if (tabBody.getAttribute("data-cdrm-group") === groupId) {
            tabBody.style.transform = `translateX(${(i - clickedIndex) * 100}%)`;
            if (i === clickedIndex) {
                tabBody.style.opacity = "1";
            } else {
                tabBody.style.opacity = "0";
            }
        }
    }
};
