function handleTabGroupRowAndHeight(container: HTMLDivElement) {
    const focusedIndexString = container.getAttribute("data-cdrm-focused-idx");
    if (typeof focusedIndexString === "undefined" || focusedIndexString === null) {
        console.error("No focused index found for the specified tab group.")
        return
    }
    const focusedIndex = parseInt(focusedIndexString);

    const bodyContainer = container.querySelector(".cdrm-tab-group-body-container") as HTMLDivElement | undefined;
    if (!bodyContainer) {
        console.error("Could not locate tab group body container.")
        return
    }

    const groupTabs = bodyContainer.getElementsByClassName("cdrm-tab-group-item");


    for (let i = 0; i < groupTabs.length; i++) {
        const tab = groupTabs.item(i) as HTMLDivElement;
        tab.style.transition = "transform 0.3s ease-in-out";
        tab.style.transform = `translateX(-${focusedIndex === 0 ? 0 : focusedIndex / groupTabs.length}%)`
    }


    const tabGroupId = container.getAttribute("data-cdrm-group");

    if (!tabGroupId) {
        console.error("Could not locate tab group id. Can't continue.")
        return
    }

    const focusedTab = bodyContainer.querySelector(`#tab-${tabGroupId}-${focusedIndexString}`);

    if (!focusedTab) {
        console.error("Cold not locate focused tab. Cannot continue with transition.")
        return
    }

    const targetHeight = Math.min(focusedTab.getBoundingClientRect().height, 450);

    bodyContainer.style.transition = "height 0.3s ease-in-out";
    bodyContainer.style.overflowY = targetHeight === 450 ? "auto" : "hidden";
    bodyContainer.style.height = `${targetHeight}px`;

}

export const onTabResize = () => {
    const containers = document.getElementsByClassName("cdrm-tab-group");
    for (let i = 0; i < containers.length; i++) {
        const tabGroup = containers.item(i) as HTMLDivElement | undefined;
        if (tabGroup) {
            handleTabGroupRowAndHeight(tabGroup);
        }
    }
};

/// If the div passed in is not a valid container this will break.
export const onTabLoad = () => {
    const containers = document.getElementsByClassName("cdrm-tab-group");
    for (let i = 0; i < containers.length; i++) {
        const tabGroup = containers.item(i) as HTMLDivElement;
        handleTabGroupRowAndHeight(tabGroup);

        const observer = new MutationObserver(() => {
            handleTabGroupRowAndHeight(tabGroup);
        });
        observer.observe(tabGroup, {
            attributes: true,
            attributeFilter: ["data-cdrm-focused-idx"],
        });
    }
};
