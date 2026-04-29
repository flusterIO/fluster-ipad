import consola from "consola";
import { onResize } from "./on_resize";

export const onLoad = () => {
    consola.info("Conundrum content loaded...");
    onResize();
};
