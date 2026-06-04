import { modifyTurboJson } from "./script_wrappers/modify_turbo_json";

modifyTurboJson((data, entity) => {
    console.log("data: ", data);
    console.log("entity: ", entity.data);

    return undefined;
});
