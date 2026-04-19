import { getRepoEntities } from "./repo_entity";
import fs from "fs";
import path from "path";

const PATH_TO_COPY = path.resolve(
    __dirname,
    "../../node_modules/katex/dist/katex.min.css",
);
const OUTPUT_PATH = "katex.min.css";

const entities = getRepoEntities();

console.log("PATH_TO_COPY: ", PATH_TO_COPY)

for (const entity of entities) {
    if (entity.data.is_webview) {
        if (!fs.existsSync(entity.publicDir())) {
            fs.mkdirSync(entity.publicDir())
        }
        entity.copyToPublicDir(PATH_TO_COPY, OUTPUT_PATH)
    }
}
