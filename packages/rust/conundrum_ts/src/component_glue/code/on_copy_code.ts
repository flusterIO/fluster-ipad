import consola from "consola";

export const onCopyCodeBlockClick = (e: Event) => {
    const target = e.currentTarget as HTMLDivElement;
    const targetId = target.getAttribute("data-cdrm-copy-for");
    if (!targetId) {
        consola.error(
            "Failed to find a valid targetID on the code block.",
            e.currentTarget,
        );
        return;
    }
    const parentEm = document.getElementById(targetId);
    if (!parentEm) {
        return;
    }
    window.navigator.clipboard
        .writeText(parentEm.querySelector("pre")?.innerText ?? "")
        .catch((err: unknown) => {
            consola.error("Copy error: ", err);
        });
    const blockContainer = document.querySelector(
        `div[data-cdrm-codeblock="${targetId}"]`,
    ) as HTMLDivElement | undefined;
    if (blockContainer) {
        const lang = blockContainer.getAttribute("data-cdrm-lang");
        window.dispatchEvent(
            new CustomEvent("cdrm-codeblock-copy", {
                detail: {
                    lang,
                },
            }),
        );
    }
};
