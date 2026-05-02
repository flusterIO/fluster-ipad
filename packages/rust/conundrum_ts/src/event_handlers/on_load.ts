import { onFootnotesLoad, onTabLoad } from "../methods";
import { onResize } from "./on_resize";

export const onLoad = () => {
    onTabLoad();
    onResize();
    onFootnotesLoad(); // Just click listeners, ok to leave towards the bottom.
    console.info("Conundrum content loaded from @conundrum/ts");
};
