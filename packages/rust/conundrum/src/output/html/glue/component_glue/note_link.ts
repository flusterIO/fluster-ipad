const handleNoteLinkClick = (e: Event) => {
    let target = e.target as HTMLAnchorElement;
    let noteId = target.getAttribute("data-note-id-referencing");
    window.dispatchEvent(new CustomEvent("cdrm-note-id-link-click", {
        detail: {
            noteId
        }
    }))
}
