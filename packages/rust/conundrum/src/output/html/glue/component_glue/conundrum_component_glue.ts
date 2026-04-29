import { handleConundrumAdmonitionResize, applyAdmonitionClickListeners } from "./admonition_utils";
import { applyCopyConundrumCodeBlockListeners } from "./code_block";
import { addConundrumTabClickListeners, handleConundrumTabGroupHeight } from "./tabs";


export const handleConundrumTabClick = () => {
    console.log(`Click registered!!`)
}

const onResize = (): void => {
    handleConundrumTabGroupHeight();
    handleConundrumAdmonitionResize();
}

export const onConundrumContentLoaded = (): void => {
    applyAdmonitionClickListeners();
    applyCopyConundrumCodeBlockListeners();
    addConundrumTabClickListeners();
}


export const initializeConundrumStaticWebAssets = () => {
    debugger;
    onResize();
    onConundrumContentLoaded();
    window.addEventListener("cdrm-content-loaded", onConundrumContentLoaded);
    window.addEventListener("resize", onResize);
    window.addEventListener("cdrm-manual-resize", onResize);
    console.info("Initialized Conundrum glue code...")
}

export const cleanupConundrumStaticWebAssets = () => {
    window.removeEventListener("cdrm-content-loaded", onConundrumContentLoaded)
    window.removeEventListener("resize", onResize);
    window.removeEventListener("cdrm-manual-resize", onResize);
}
