const getElementRecursively = (
    em: HTMLDivElement,
): [string, HTMLDivElement] | undefined => {
    const groupId = em.getAttribute("data-cdrm-group");
    if (groupId) {
        return [groupId, em];
    }
    const parent = em.parentElement as HTMLDivElement | undefined;
    if (parent) {
        return getElementRecursively(parent);
    }
};

const getTargetTabIndex = (em: HTMLDivElement): number | undefined => {
    const n = em.getAttribute("data-cdrm-idx");
    if (typeof n !== "undefined" && n !== null) {
        return parseInt(n)
    }
}

export const handleConundrumTabClick = (e: Event) => {
    const target = e.currentTarget as HTMLDivElement;

    const clickedIndex = getTargetTabIndex(target);
    if (typeof clickedIndex === "undefined") {
        console.error("Could not find tab index.");
        return;
    }

    const _em = getElementRecursively(target);
    if (!_em) {
        console.error("Could not find proper parent element.");
        return;
    }
    const [groupId, em] = _em;

    const focusedIdx = em.getAttribute("data-cdrm-focused-idx");
    if (typeof focusedIdx === "undefined") {
        console.error("Could not found TabGroup focused index.");
        return;
    }
    /* eslint-disable-next-line  -- I just checkedd.... I'm starting to hate eslint. */
    const lastFocusedIndex = parseInt(focusedIdx!);

    const tabs = em.querySelectorAll(".cdrm-tab-subtle-border");
    for (let i = 0; i < tabs.length; i++) {
        const tab = tabs.item(i) as HTMLDivElement;
        const bgClasses = tab.classList
            .values()
            .toArray()
            .filter((s) => s.startsWith("bg-"));
        for (const k of bgClasses) {
            tab.classList.remove(k);
        }
        if (i === clickedIndex) {
            const activeTabBorder = target.querySelector(
                ".cdrm-tab-subtle-border",
            ) as HTMLDivElement | undefined;

            if (activeTabBorder) {
                activeTabBorder.style.transformOrigin =
                    lastFocusedIndex < clickedIndex ? "left" : "right";
                activeTabBorder.classList.remove("bg-transparent");
                activeTabBorder.classList.remove("scale-x-0");
                const emphasis = em.getAttribute("data-cdrm-emphasis");
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

    const allTabBodies = em.getElementsByClassName("cdrm-tab-group-item");

    for (let i = 0; i < allTabBodies.length; i++) {
        const tabBody = allTabBodies.item(i) as HTMLDivElement;
        if (tabBody.getAttribute("data-cdrm-group") === groupId) {
            tabBody.classList.remove("relative");
            tabBody.classList.add("absolute");
            tabBody.classList.remove("overflow-y-hidden");
            // tabBody.style.transform = `translateX(${(i - clickedIndex) * 100}%)`;
            if (i === clickedIndex) {
                tabBody.style.opacity = "1";
            } else {
                tabBody.style.opacity = "0";
            }
        }
    }
};
