import { consola } from "consola";
import fs, { readFileSync } from "fs";
import { globSync } from "glob";
import { z } from "zod";

const packageJsonSchema = z.object({
    name: z.string(),
    // name: z.string().refine((n) => n.startsWith("@fluster")),
    version: z.string(),
    packageManager: z.literal("pnpm@9.7.1"),
    // scripts: z.record(z.string(), z.string()),
    scripts: z.any(),
});
export type PackageJsonPartial = z.infer<typeof packageJsonSchema>;

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
