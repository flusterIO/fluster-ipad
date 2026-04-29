export const onTabResize = () => {
    function removeInitialRelativePositions(container: HTMLDivElement) {
        const tabs = container.querySelectorAll(".cdrm-tab-group-item");
        for (let i = 0; i < tabs.length; i++) {
            const item = tabs.item(i) as HTMLDivElement | undefined;
            if (item?.classList.contains("relative")) {
                item.classList.remove("relative");
                item.classList.add("absolute");
            }
        }
    }
    /// If the div passed in is not a valid container this will break.
    function handleHeight(container: HTMLDivElement) {
        const focusedIndex = parseInt(
            /* eslint-disable-next-line  -- It'll be there... I put it there. */
            container.getAttribute("data-cdrm-focused-idx")!,
        );
        const groupId = container.getAttribute("data-cdrm-group");
        if (!groupId) {
            console.warn(
                "Compiler Error: Found a tab group without a valid group id.",
            );
            return;
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
                bodyWrapper.style.transition = "none";
            }
        }
    }
    const containers = document.getElementsByClassName("cdrm-tab-group");
    for (let i = 0; i < containers.length; i++) {
        const tabGroup = containers.item(i) as HTMLDivElement;

        const observer = new MutationObserver(() => {
            handleHeight(tabGroup);
        });
        observer.observe(tabGroup, {
            attributes: true,
            attributeFilter: ["data-cdrm-focused-idx"],
        });
        handleHeight(tabGroup);
        removeInitialRelativePositions(tabGroup);
    }
};
