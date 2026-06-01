export const handleNoteLinkClick = (e: Event) => {
    const target = e.target as HTMLAnchorElement;
    const noteId = target.getAttribute("data-note-id-referencing");
    window.dispatchEvent(
        new CustomEvent("cdrm-note-id-link-click", {
            detail: {
                noteId,
            },
        }),
    );
};
