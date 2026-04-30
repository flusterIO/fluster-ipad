import { handleConundrumAdmonitionHeight } from "src/component_glue/methods";

export const onResize = (e?: Event) => {
    console.info("Resize Event: ", e);

    const admonitions = document.getElementsByClassName("cdrm-admon");
    for (let l = 0; l < admonitions.length; l++) {
        const admon = admonitions.item(l) as HTMLDivElement | undefined;
        if (admon) {
            handleConundrumAdmonitionHeight(admon);
        }
    }
};
