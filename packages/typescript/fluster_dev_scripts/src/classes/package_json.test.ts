import { describe, it, expect } from "vitest";
import { PackageJson } from "../classes/package_json";
import { consola } from "consola";
import { allMonorepoPackages } from "../classes/monorepo_entities";

describe("Package.json behaves as expected. It's embarassing to even need to TDD something so simple but I'm tired as shit...", () => {
    it("Gathers all monorepo files and nothing else", () => {
        const packages = PackageJson.getAllPackageJsons();
        for (const k of packages) {
            const monoRepoExists = allMonorepoPackages.some(
                (a) => new a().rootDir === k.projectRootDir().rootDir,
            );
            expect(
                monoRepoExists,
                "Found a monorepo project not in the `packages/typescript/fluster_dev_scripts/src/classes/monorepo_entities.ts` file.",
            );
        }
        consola.success("Finally no errors...");
    });
});
