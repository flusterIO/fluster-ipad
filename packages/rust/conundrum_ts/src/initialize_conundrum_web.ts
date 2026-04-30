import { handleConundrumTabClick } from "./component_glue/tabs/handle_tab_click";
import { onCopyCodeBlockClick } from "./component_glue/code/on_copy_code";
import { onCodeBlockContainerClick } from "./component_glue/code/on_codeblock_container_click";
import { onAdmonitionHeadingClick } from "./component_glue/admonition/on_admonition_heading_click";
import { type ConundrumWebClient } from "./conundrum_web";
import { onLoad } from "./event_handlers/on_load";
import { onResize } from "./event_handlers/on_resize";

declare global {
    interface Window {
        conundrum: ConundrumWebClient;
    }
}

const handleContainerMutations = (container: HTMLDivElement) => {
    const haveSet = container.getAttribute("data-cdrm-mutated") === "true";
    if (haveSet) {
        return;
    }
    /// Document was just reloaded... do your thing.
    onLoad();

    container.setAttribute("data-cdrm-mutated", "true");
};

const handleMutations = (): void => {
    const container = document.getElementById("cdrm-body-container") as
        | HTMLDivElement
        | undefined;
    if (container) {
        handleContainerMutations(container);
    }
};

export const initializeConundrumWeb = () => {
    window.conundrum = {
        handleConundrumTabClick,
        onCodeBlockContainerClick,
        onCopyCodeBlockClick,
        onAdmonitionHeadingClick,
        onLoad,
        onResize,
    };
    const mainObserver = new MutationObserver(handleMutations);

    mainObserver.observe(document.body, {
        childList: true,
        attributes: true,
        attributeFilter: ["data-cdrm-mutated"],
        subtree: true,
    });

    window.addEventListener("cdrm-manual-resize", onResize);
};
