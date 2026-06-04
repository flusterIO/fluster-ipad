import consola from "consola";
import {
    getRepoEntities,
    type TurboJsonType,
    type RepoEntity,
} from "../repo_entity";
import fs from "fs";

export const modifyTurboJson = (
    cb: (
        data: Partial<TurboJsonType>,
        entity: RepoEntity,
    ) => Partial<TurboJsonType> | undefined,
) => {
    const entities = getRepoEntities();
    for (const entity of entities) {
        const data = entity.readTurbo();
        const res: Partial<TurboJsonType> | undefined = cb(data, entity);
        if (res) {
            const turboPath = entity.turboPath();
            fs.writeFileSync(turboPath, JSON.stringify(res, null, 2), {
                encoding: "utf-8",
            });
            consola.info(`Successfully wrote Turbo.json to ${turboPath}`);
        }
    }
};
