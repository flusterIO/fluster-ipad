import { ConundrumWebEvents, type CopyToClipboardSource } from "../code_gen";

interface CdrmContentCopiedProps {
    source: CopyToClipboardSource;
    content: string;
}

declare global {
    interface WindowEventMap {
        [ConundrumWebEvents.CopyToClipboard]: CustomEvent<CdrmContentCopiedProps>;
    }
}

export const copyStringToClipboard = async (
    content: string,
    source: CopyToClipboardSource,
) => {
    await window.navigator.clipboard.writeText(content);

    window.dispatchEvent(
        new CustomEvent(ConundrumWebEvents.CopyToClipboard as string, {
            detail: {
                source,
                content,
            },
        }),
    );
};
