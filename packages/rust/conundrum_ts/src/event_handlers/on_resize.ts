import { onAdmonitionResize } from "../component_glue/admonition/on_admonition_resize";
import { onTabResize } from "../methods";

export const onResize = () => {
    onTabResize();
    onAdmonitionResize();
};
