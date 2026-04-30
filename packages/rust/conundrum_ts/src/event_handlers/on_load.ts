import { onTabLoad } from "src/component_glue/methods";
import { onResize } from "./on_resize";

export const onLoad = () => {
    console.info("Conundrum content loaded from @conundrum/ts");
    onTabLoad();
    onResize();
};
