const onFootnoteAnchorClick = (e: Event) => {
    const target = e.currentTarget as HTMLElement;
    const footer = document.querySelector(`div[data-cdrm-for="${target.id}"]`);
    if (!footer) {
        console.error("Could not find footer for footnote anchor");
        return;
    }
    footer.scrollIntoView({
        behavior: "smooth",
    });
};

const onFootnoteClick = (e: Event) => {
    const target = e.currentTarget as HTMLAnchorElement;
    const anchorId = target.getAttribute("data-cdrm-for");
    if (!anchorId) {
        console.error("Could not find anchor id.");
        return;
    }
    const anchor = document.getElementById(anchorId);
    if (!anchor) {
        console.error("Could not find an assocated anchor the footnote footer.");
        return;
    }
    anchor.scrollIntoView({
        behavior: "smooth",
    });
};

export const onFootnotesLoad = () => {
    const ems = document.getElementsByClassName("cdrm-footnote");
    for (let i = 0; i < ems.length; i++) {
        const item = ems.item(i);
        if (item) {
            item.addEventListener("click", onFootnoteClick);
        }
    }
    const anchors = document.getElementsByClassName("cdrm-footnote-anchor");
    for (let i = 0; i < anchors.length; i++) {
        const item = anchors.item(i);
        if (item) {
            item.addEventListener("click", onFootnoteAnchorClick);
        }
    }
};
