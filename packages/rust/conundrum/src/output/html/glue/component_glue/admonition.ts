import { applyAdmonitionClickListeners, handleConundrumAdmonitionResize } from "./admonition_utils";

// This will need to be builit with vite-single-file as an independent entry point, as will all other files that have been split into two to work with the app-embedded output.
(() => {
    applyAdmonitionClickListeners()
    window.addEventListener("resize", handleConundrumAdmonitionResize);
    window.addEventListener("cdrm-manual-resize", handleConundrumAdmonitionResize);
    window.addEventListener("cdrm-content-loaded", applyAdmonitionClickListeners)
})();
