
export const showBibEntryDetailsById = (id: string) => {
    window.dispatchEvent(
        new CustomEvent("show-bib-entry-details", {
            detail: {
                itemId: id,
            },
        })
    );
};
