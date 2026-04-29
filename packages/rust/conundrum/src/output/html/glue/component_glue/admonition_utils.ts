// This logic was duplicated in the conundrum/ts crate. Delete this and import things directly from there.
export function handleConundrumAdmonitionHeight(container: HTMLDivElement) {
    const bodyContainer = container.querySelector(".cdrm-admon-body-container") as HTMLDivElement;
    const body = container.querySelector(".cdrm-admon-body") as HTMLDivElement;
    body.style.height = 'auto';
    body.style.transition = 'max-height 500ms ease-in-out';
    const bodyHeight = body.getBoundingClientRect().height;
    bodyContainer.style.maxHeight = `${bodyHeight}px`
}

export function applyAdmonitionClickListeners() {

    let ems = document.getElementsByClassName("cdrm-admon-title-group") as HTMLCollectionOf<HTMLDivElement>;
    for (var i = 0; i < ems.length; i++) {
        const item = ems.item(i);
        item?.addEventListener("click", (e) => {

        })
    }
}


export function handleConundrumAdmonitionResize() {
    const ems = document.getElementsByClassName("cdrm-admon");
    for (var i = 0; i < ems.length; i++) {
        const item = ems.item(i) as HTMLDivElement;
        if (item.getAttribute("data-cdrm-folded") === "false" && item.getAttribute("data-cdrm-foldable") === "true") {
            handleConundrumAdmonitionHeight(item)
        }
    }
}
