export function handleConundrumAdmonitionHeight(container: HTMLDivElement) {
    const bodyContainer = container.querySelector(
        ".cdrm-admon-body-container",
    ) as HTMLDivElement | undefined;
    if (!bodyContainer) {
        return;
    }
    const body = container.querySelector(".cdrm-admon-body") as
        | HTMLDivElement
        | undefined;
    if (!body) {
        return;
    }
    body.style.height = "auto";
    body.style.transition = "max-height 500ms ease-in-out";
    const bodyHeight = body.getBoundingClientRect().height;
    bodyContainer.style.maxHeight = `${bodyHeight}px`;
}

export const onAdmonitionHeadingClick = (e: Event) => {
    function openAdmonition(container: HTMLDivElement) {
        container.setAttribute("data-cdrm-folded", "false");
        handleConundrumAdmonitionHeight(container);
    }

    function closeAdmonition(container: HTMLDivElement) {
        const bodyContainer = container.querySelector(
            ".cdrm-admon-body-container",
        ) as HTMLDivElement | undefined;
        if (!bodyContainer) {
            return;
        }
        bodyContainer.style.maxHeight = "0px";
        container.setAttribute("data-cdrm-folded", "true");
    }

    const container = (e.currentTarget as HTMLDivElement)
        .parentElement as HTMLDivElement;
    const folded = container.getAttribute("data-cdrm-folded") === "true";
    const foldable = container.getAttribute("data-cdrm-foldable") === "true";
    const body = container.querySelector(".cdrm-admon-body-container");
    if (!body) {
        console.error("Could not find admonition body");
        return;
    }
    if (foldable) {
        if (folded) {
            openAdmonition(container);
        } else {
            closeAdmonition(container);
        }
    }
};
