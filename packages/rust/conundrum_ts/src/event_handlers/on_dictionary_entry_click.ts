import { ConundrumWebEvents } from "../code_gen";

export const onDictionaryEntryClick = (e: Event) => {
    const target = e.currentTarget as HTMLDivElement;
    const noteId = target.getAttribute("data-cdrm-noteid");
    if (noteId) {
        window.dispatchEvent(new CustomEvent(ConundrumWebEvents.DictionaryEntryLabelClick))
    }

}
