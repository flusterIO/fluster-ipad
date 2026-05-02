import { onFootnotesLoad, onTabLoad } from "../methods";
import { onResize } from "./on_resize";

export const onLoad = () => {
    console.info("Conundrum content loaded from @conundrum/ts");
    onTabLoad();
    onResize();
    onFootnotesLoad(); // Just click listeners, ok to leave towards the bottom.
};
