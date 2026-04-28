(() => {
    function handleTocHeadingClick(e) {
        const target = e.currentTarget;
        const heading = document.getElementById(`h-${target.getAttribute("cdrm-heading-id")}`);
        if (heading) {
            heading.scrollIntoView({
                behavior: "smooth"
            });
        }
    }
    const items = document.getElementsByClassName("cdrm-toc-item");
    for (var i = 0; i < items.length; i++) {
        const item = items.item(i);
        item.addEventListener("click", handleTocHeadingClick);
    }
})();

(() => {
    function handleAdmonitionHeight(container) {
        const bodyContainer = container.querySelector(".cdrm-admon-body-container");
        const body = container.querySelector(".cdrm-admon-body");
        body.style.height = 'auto';
        body.style.transition = 'max-height 500ms ease-in-out';
        const bodyHeight = body.getBoundingClientRect().height;
        bodyContainer.style.maxHeight = `${bodyHeight}px`;
    }
    function openAdmonition(container) {
        container.setAttribute("data-cdrm-folded", "false");
        handleAdmonitionHeight(container);
    }
    function closeAdmonition(container) {
        const bodyContainer = container.querySelector(".cdrm-admon-body-container");
        bodyContainer.style.maxHeight = "0px";
        container.setAttribute("data-cdrm-folded", "true");
    }
    let ems = document.getElementsByClassName("cdrm-admon-title-group");
    for (var i = 0; i < ems.length; i++) {
        const item = ems.item(i);
        item.addEventListener("click", (e) => {
            const container = e.currentTarget.parentElement;
            let folded = container.getAttribute("data-cdrm-folded") === "true";
            let foldable = container.getAttribute("data-cdrm-foldable") === "true";
            const body = container.querySelector(".cdrm-admon-body-container");
            if (!body) {
                return;
            }
            if (foldable) {
                if (folded) {
                    openAdmonition(container);
                }
                else {
                    closeAdmonition(container);
                }
            }
        });
    }
    function handleAdmonitionResize() {
        const ems = document.getElementsByClassName("cdrm-admon");
        for (var i = 0; i < ems.length; i++) {
            const item = ems.item(i);
            if (item.getAttribute("data-cdrm-folded") === "false" && item.getAttribute("data-cdrm-foldable") === "true") {
                handleAdmonitionHeight(item);
            }
        }
    }
    window.addEventListener("resize", handleAdmonitionResize);
    window.addEventListener("cdrm-manual-resize", handleAdmonitionResize);
})();

(() => {
    function handleTabClick(e) {
        const em = e.currentTarget.parentElement.parentElement;
        if (!em) {
            return;
        }
        const emphasis = em.getAttribute("data-cdrm-emphasis");
        let tabs = em.querySelectorAll(".cdrm-tab-subtle-border");
        const clickedIndex = parseInt(e.currentTarget.getAttribute("data-cdrm-idx"));
        const groupId = em.getAttribute("data-cdrm-group");
        const lastFocusedIndex = parseInt(em.getAttribute("data-cdrm-focused-idx"));
        for (var i = 0; i < tabs.length; i++) {
            const tab = tabs.item(i);
            let bgClasses = tab.classList.values().toArray().filter((s) => s.startsWith("bg-"));
            for (const k of bgClasses) {
                tab.classList.remove(k);
            }
            if (i === clickedIndex) {
                let activeTabBorder = e.currentTarget.querySelector(".cdrm-tab-subtle-border");
                if (activeTabBorder) {
                    activeTabBorder.style.transformOrigin = lastFocusedIndex < clickedIndex ? "left" : "right";
                    activeTabBorder.classList.remove("bg-transparent");
                    activeTabBorder.classList.remove("scale-x-0");
                    // DO NOT SCAN THESE FILES WITH TAILWIND OR SHIT WILL EXPLODE
                    activeTabBorder.classList.add(`bg-emphasis-${emphasis}`);
                }
            }
            else {
                tab.style.transformOrigin = lastFocusedIndex > clickedIndex ? "left" : "right";
                tab.classList.add("bg-transparent");
                tab.classList.add("scale-x-0");
            }
        }
        em.setAttribute("data-cdrm-focused-idx", `${clickedIndex}`);
        const allTabBodies = document.getElementsByClassName("cdrm-tab-group-item");
        for (var i = 0; i < allTabBodies.length; i++) {
            const tabBody = allTabBodies.item(i);
            if (tabBody.getAttribute("data-cdrm-group") === groupId) {
                tabBody.style.transform = `translateX(${(i - clickedIndex) * 100}%)`;
                if (i === clickedIndex) {
                    tabBody.style.opacity = "1";
                }
                else {
                    tabBody.style.opacity = "0";
                }
            }
        }
    }
    /// If the div passed in is not a valid container this will break.
    function handleHeight(container) {
        const focusedIndex = parseInt(container.getAttribute("data-cdrm-focused-idx"));
        const groupId = container.getAttribute("data-cdrm-group");
        const focusedTabBody = container.querySelector(`#tab-${groupId}-${focusedIndex}`);
        if (focusedTabBody) {
            const h = focusedTabBody.getBoundingClientRect().height;
            const bodyWrapper = container.querySelector(`#tab-body-wrapper-${groupId}`);
            if (bodyWrapper) {
                bodyWrapper.style.transition = "height 0.3s ease-in-out";
                bodyWrapper.style.height = `${Math.min(h, 450)}px`;
            }
        }
    }
    const ems = document.getElementsByClassName("cdrm-tab-btn");
    for (var i = 0; i < ems.length; i++) {
        const item = ems.item(i);
        item.addEventListener("click", handleTabClick);
    }
    const gatheHeightOfAllTabGroups = () => {
        const containers = document.getElementsByClassName("cdrm-tab-group");
        for (var i = 0; i < containers.length; i++) {
            const tabGroup = containers.item(i);
            const observer = new MutationObserver(() => {
                handleHeight(tabGroup);
            });
            observer.observe(tabGroup, {
                attributes: true,
                attributeFilter: ["data-cdrm-focused-idx"]
            });
            handleHeight(tabGroup);
        }
    };
    gatheHeightOfAllTabGroups();
    window.addEventListener("resize", gatheHeightOfAllTabGroups);
})();

