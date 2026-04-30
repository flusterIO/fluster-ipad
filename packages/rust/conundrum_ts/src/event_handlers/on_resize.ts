import { onAdmonitionResize } from "src/component_glue/admonition/on_admonition_resize";
import { onTabResize } from "src/component_glue/methods";

export const onResize = () => {
    onTabResize();
    onAdmonitionResize();
};
