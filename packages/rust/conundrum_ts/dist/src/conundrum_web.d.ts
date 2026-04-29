import { handleConundrumTabClick } from './component_glue/tabs/handle_tab_click';
import { onCopyCodeBlockClick } from './component_glue/code/on_copy_code';
import { onCodeBlockContainerClick } from './component_glue/code/on_codeblock_container_click';
import { onAdmonitionHeadingClick } from './component_glue/admonition/on_admonition_heading_click';
export interface ConundrumWebClient {
    handleConundrumTabClick: typeof handleConundrumTabClick;
    onAdmonitionHeadingClick: typeof onAdmonitionHeadingClick;
    onCopyCodeBlockClick: typeof onCopyCodeBlockClick;
    onCodeBlockContainerClick: typeof onCodeBlockContainerClick;
}
//# sourceMappingURL=conundrum_web.d.ts.map