import { ConundrumWebEvents } from "../code_gen";

interface TagClickEventProps {
    tagBody: string;
}
declare global {
    interface WindowEventMap {
        [ConundrumWebEvents.TagClick]: CustomEvent<TagClickEventProps>;
    }
}

export const handleTagClick = (e: Event): void => {
    const tagBody = (e.currentTarget as HTMLDivElement).getAttribute(
        "data-cdrm-tag-body",
    );
    if (!tagBody) {
        console.error("Could not locate tag body.");
        return;
    }
    window.dispatchEvent(
        new CustomEvent(ConundrumWebEvents.TagClick, {
            detail: {
                tagBody,
            },
        }),
    );
};
