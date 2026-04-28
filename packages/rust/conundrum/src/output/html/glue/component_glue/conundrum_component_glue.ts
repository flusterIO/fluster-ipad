import { applyAdmonitionClickListeners, handleConundrumAdmonitionResize } from "./admonition"
import { applyCopyConundrumCodeBlockListeners } from "./code_block";
import { addConundrumTabClickListeners, handleConundrumTabGroupHeight } from "./tabs";



const onResize = (): void => {
    handleConundrumTabGroupHeight();
    handleConundrumAdmonitionResize();
}

export const onConunundrumContentLoaded = (): void => {
    handleConundrumAdmonitionResize(); // UI feature... keep at the front
    handleConundrumTabGroupHeight();// UI feature... keep at the front
    applyAdmonitionClickListeners();
    applyCopyConundrumCodeBlockListeners();
    addConundrumTabClickListeners();
}

export const initializeConundrumStaticWebAssets = () => {
    onConunundrumContentLoaded();
    onResize();
    window.addEventListener("cdrm-content-loaded", onConunundrumContentLoaded);
    window.addEventListener("resize", onResize);
    window.addEventListener("cdrm-manual-resize", onResize);
    console.info("Initialized Conundrum glue code...")
}

export const cleanupConundrumStaticWebAssets = () => {
    window.removeEventListener("cdrm-content-loaded", onConunundrumContentLoaded)
    window.removeEventListener("resize", onResize);
    window.removeEventListener("cdrm-manual-resize", onResize);
}
