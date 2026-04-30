import { handleConundrumAdmonitionHeight } from "./on_admonition_heading_click";

export const onAdmonitionResize = (): void => {
    const admonitions = document.getElementsByClassName("cdrm-admon");
    for (let l = 0; l < admonitions.length; l++) {
        const admon = admonitions.item(l) as HTMLDivElement | undefined;
        if (admon) {
            handleConundrumAdmonitionHeight(admon);
        }
    }
};
