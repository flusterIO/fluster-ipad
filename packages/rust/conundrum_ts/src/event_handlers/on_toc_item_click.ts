import { ConundrumWebEvents } from "../code_gen";

declare global {

    interface WindowEventMap {
        [ConundrumWebEvents.TocItemClick]: CustomEvent;
    }
}


export const onTocItemClick = (e: Event) => {
    const target = e.currentTarget as HTMLDivElement;
    const headingId = target.getAttribute("data-cdrm-heading-id");
    if (headingId) {
        const heading = document.getElementById(headingId);
        if (heading) {
            heading.scrollIntoView({
                behavior: "smooth"
            })
        } else {
            console.warn(`Requested a heading with the id of ${headingId}. This does not exist.`)
        }
    }

    window.dispatchEvent(new CustomEvent(ConundrumWebEvents.TocItemClick as string))

}
