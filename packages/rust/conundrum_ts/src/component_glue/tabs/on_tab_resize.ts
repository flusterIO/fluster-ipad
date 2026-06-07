function handleTabGroupRowAndHeight(container: HTMLDivElement) {
    const focusedIndex = parseInt(
        /* eslint-disable-next-line  -- It'll be there... I put it there. */
        container.getAttribute("data-cdrm-focused-idx")!,
    );
    const groupId = container.getAttribute("data-cdrm-group");
    if (!groupId) {
        console.warn("Compiler Error: Found a tab group without a valid group id.");
        return;
    }
    const gridRow = container.getElementsByClassName("cdrm-tab-row");
    if (!gridRow.length) {
        console.error("Could not locate grid row in tabs component. Don't know how to continue with transition.")
    } else {
        const gridRowElement = gridRow.item(0) as HTMLDivElement;
        gridRowElement.style.transition = "transition 0.3s ease-in-out";
        gridRowElement.style.transform = `translateX(-${container.getBoundingClientRect().width * focusedIndex}px)`;
    }
    const focusedTabBody = container.querySelector(
        `#tab-${groupId}-${focusedIndex}`,
    ) as HTMLDivElement | undefined;
    if (focusedTabBody) {
        const h = focusedTabBody.getBoundingClientRect().height;
        const bodyWrapper = container.querySelector(
            `#tab-body-wrapper-${groupId}`,
        ) as HTMLDivElement | undefined;
        if (bodyWrapper) {
            bodyWrapper.style.transition = "height 0.3s ease-in-out";
            bodyWrapper.style.height = `${Math.min(h, 450)}px`;
            bodyWrapper.style.overflowY = h > 450 ? "auto" : "hidden";
        } else {
            console.error("Could not find tab body wrapper.");
        }
    } else {
        console.error("Could not find focused body");
    }
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
