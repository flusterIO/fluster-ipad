import { type handleConundrumTabClick } from "./component_glue/tabs/handle_tab_click";
import { type onCopyCodeBlockClick } from "./component_glue/code/on_copy_code";
import { type onCodeBlockContainerClick } from "./component_glue/code/on_codeblock_container_click";
import { type onAdmonitionHeadingClick } from "./component_glue/admonition/on_admonition_heading_click";
import { type onLoad } from "./event_handlers/on_load";
import { type onResize } from "./event_handlers/on_resize";

export interface ConundrumWebClient {
    handleConundrumTabClick: typeof handleConundrumTabClick;
    onAdmonitionHeadingClick: typeof onAdmonitionHeadingClick;
    onCopyCodeBlockClick: typeof onCopyCodeBlockClick;
    onCodeBlockContainerClick: typeof onCodeBlockContainerClick;
    onResize: typeof onResize;
    onLoad: typeof onLoad;
}
