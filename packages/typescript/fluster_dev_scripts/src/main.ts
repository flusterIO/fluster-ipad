import { Command } from "commander";
const program = new Command();
import { PackageJson } from "./classes/package_json";

program
    .name("fluster-dev")
    .description("Internal Fluster development scripts")
    .version("0.0.0");

program
    .command("modify-package-jsons")
    .description(
        "Modifies each package.json file according to the default export in packages/typescript/fluster_dev_scripts/inputs/MODIFY_PACKAGE_JSONS.ts file. This function accepts an array of all monorepo packageons, must return that array, and all items that are returned are then saved. You don't need to return all entries, but you must return an array.",
    )
    .action(async () => {
        const f = await import("../inputs/MODIFY_PACKAGE_JSONS").then(
            (a) => a.default,
        );
        const packageJsons = PackageJson.getAllPackageJsons();
        const newPackageJsons = f(packageJsons);
        for (const p of newPackageJsons) {
            p.writeDataToFile();
        }
    });

program.parse();
