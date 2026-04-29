import { handleConundrumTabClick } from "./component_glue/tabs/handle_tab_click";
import { onCopyCodeBlockClick } from "./component_glue/code/on_copy_code";
import { onCodeBlockContainerClick } from "./component_glue/code/on_codeblock_container_click";
import {
  onAdmonitionHeadingClick,
  handleConundrumAdmonitionHeight,
} from "./component_glue/admonition/on_admonition_heading_click";
import { type ConundrumWebClient } from "./conundrum_web";

declare global {
  interface Window {
    conundrum: ConundrumWebClient;
  }
}

export const initializeConundrumWeb = () => {
  window.conundrum = {
    handleConundrumTabClick,
    onCodeBlockContainerClick,
    onCopyCodeBlockClick,
    onAdmonitionHeadingClick,
  };

  const onResize = (): void => {
    const admonitions = document.getElementsByClassName("cdrm-admon");
    for (let l = 0; l < admonitions.length; l++) {
      const admon = admonitions.item(l) as HTMLDivElement | undefined;
      if (admon) {
        handleConundrumAdmonitionHeight(admon);
      }
    }
  };

  window.addEventListener("resize", onResize);
  window.addEventListener("cdrm-manual-resize", onResize);
};
