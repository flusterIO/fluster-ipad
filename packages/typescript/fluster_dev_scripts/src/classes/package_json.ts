import { consola } from "consola";
import fs, { readFileSync } from "fs";
import { globSync } from "glob";
import path from "path";
import { z } from "zod";

const packageJsonSchema = z.object({
    name: z.string(),
    // name: z.string().refine((n) => n.startsWith("@fluster")),
    version: z.string(),
    packageManager: z.literal("pnpm@9.7.1"),
    // scripts: z.record(z.string(), z.string()),
    scripts: z.any(),
});

/**
 * The *value* of each item is a query to be usred in a
 * regular expression while retrieving the file, not nencessarily
 * the name of the file.
 */
export enum SpecialFile {
    eslint = "eslint",
}

export type PackageJsonPartial = z.infer<typeof packageJsonSchema>;

export class ProjectRootDir {
    rootDir: string;
    constructor(rootDir: string) {
        this.rootDir = rootDir;
    }

    read_dir() {
        return fs.readdirSync(this.rootDir);
    }

    getSpecialFilePath(specialFile: SpecialFile) {
        const reg = new RegExp(specialFile, "gi");
        const file = this.read_dir().find((f) => reg.test(f));
        if (!file) {
            consola.warn(
                `No ${specialFile} match found for the ${this.rootDir} directory.`,
            );
            return;
        }
        return path.join(this.rootDir, file);
    }

    readSpecialFile(specialFile: SpecialFile) {
        const file = this.getSpecialFilePath(specialFile);
        if (!file) {
            return;
        }
        return fs.readFileSync(file!, {
            encoding: "utf-8",
        });
    }
    writeSpecialFile(specialFile: SpecialFile, content: string) {
        const filePath = this.getSpecialFilePath(specialFile);
        if (!filePath) {
            return;
        }
        fs.writeFileSync(filePath, content, { encoding: "utf-8" });
    }

    writeDirRelativeFile(file: string, content: string) {
        fs.writeFileSync(path.join(this.rootDir, file), content, {
            encoding: "utf-8",
        });
    }
}

export class PackageJson {
    data: PackageJsonPartial;
    fp: string;
    exitOnZodFail: true;
    constructor(fp: string) {
        if (!fs.existsSync(fp)) {
            throw Error(`The ${fp} path does not exist.`);
        }
        this.fp = fp;
        const data = JSON.parse(fs.readFileSync(fp, { encoding: "utf-8" }));
        try {
            consola.info(fp);
            const parsedData = packageJsonSchema.passthrough().parse(data);
            this.data = parsedData;
        } catch (err) {
            consola.fatal(`Something went wront with the package.json file at ${fp}`);
            consola.error(`Error: ${err}`);
            if (this.exitOnZodFail) {
                throw new Error();
            } else {
                this.data = data;
            }
        }
    }

    projectRootDir(): ProjectRootDir {
        return new ProjectRootDir(path.dirname(this.fp));
    }

    validatePath(fp: string) {
        if (!fs.existsSync(fp)) {
            throw Error(`The ${fp} path does not exist.`);
        }
    }

    writeDataToFile() {
        this.validatePath(this.fp);
        fs.writeFileSync(this.fp, JSON.stringify(this.data, null, 2), {
            encoding: "utf-8",
        });
    }

    readRawFileContent(): object {
        return JSON.parse(readFileSync(this.fp, { encoding: "utf-8" }));
    }

    static getAllPackageJsons(): PackageJson[] {
        const root = process.env.FLUSTER_IOS_ROOT;
        if (!root) {
            consola.fatal(
                "The `FLUSTER_IOS_ROOT` variable was not found. Please set this variable to the root of the monorepo.",
            );
        }
        const files = globSync(
            [
                `${root!.endsWith("/") ? root : `${root}/`}apps/**/package.json`,
                `${root!.endsWith("/") ? root : `${root}/`}packages/**/package.json`,
            ],
            {
                ignore: ["**/node_modules/**"],
            },
        );
        // The extra filer method here is requird. globSync isn't behainv as expected.
        return files
            .filter((p) => !p.includes("node_modules"))
            .map((n) => new PackageJson(n));
    }
}
