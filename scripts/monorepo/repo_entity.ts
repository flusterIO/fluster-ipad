import path from "path";
import fs from "fs";
import { globSync } from "fs";

interface PackageJsonType {
    name: string;
    dependences?: Record<string, string>;
    devDependences?: Record<string, string>;
    peerDependences?: Record<string, string>;
    optionalDependences?: Record<string, string>;
}

interface RepoEntityData {
    /**
     * The absolute path
     */
    path: string;
    type: "app" | "package";
    is_webview: boolean;
    languages: ("ts" | "rust" | "swift")[];
    /**
     * True only for fluster_core_utilities and webview_utils as the kind of dumping ground for all generated code in those languages.
     */
    is_utils_package: boolean;
}

export class RepoEntity {
    public data: RepoEntityData;
    constructor(data: Omit<RepoEntityData, "languages">) {
        this.data = {
            ...data,
            languages: []
        }
        this.data.languages = this.getLanguages()
    }
    packageJsonPath(): string {
        return path.join(this.data.path, "package.json");
    }

    cargoPath(): string {
        return path.join(this.data.path, "cargo.toml");
    }

    publicDir(): string {
        return path.join(this.data.path, "public");
    }

    writePublicDir(content: string, nested_path: string) {
        fs.writeFileSync(path.resolve(this.publicDir(), nested_path), content, {
            encoding: "utf-8",
        });
    }


    copyToPublicDir(input_path: string, output_path: string) {
        const content = fs.readFileSync(input_path, {
            encoding: "utf-8"
        });
        this.writePublicDir(content, output_path)
    }

    getLanguages(): RepoEntityData["languages"] {
        const items: RepoEntityData["languages"] = [];
        if (fs.existsSync(this.cargoPath())) {
            items.push("rust");
        }
        if (fs.existsSync(this.packageJsonPath())) {
            items.push("ts");
        }
        return items;
    }

    pipePackageJson(cb: (data: PackageJsonType) => PackageJsonType | false) {
        const content = fs.readFileSync(this.packageJsonPath(), {
            encoding: "utf-8",
        });
        const ob = JSON.parse(content) as PackageJsonType;
        const res = cb(ob);
        if (res) {
            fs.writeFileSync(this.packageJsonPath(), JSON.stringify(res, null, 2), {
                encoding: "utf-8",
            });
        }
    }
}

export const getRepoEntities = () => {
    const root = process.env.FLUSTER_IOS_ROOT;
    if (!root) {
        console.error(
            "Cannot gather monorepo components without a FLUSTER_IOS_ROOT variable set.",
        );
        return;
    }
    const entities = [
        new RepoEntity({
            path: path.join(root, "packages/webview_utils"),
            type: "package",
            is_webview: true,
            is_utils_package: true,
        }),
        ...globSync(`${root}${root.endsWith("/") ? "" : "/"}apps/*`).map((n) => {
            return new RepoEntity({
                path: n,
                type: "app",
                is_webview: false,
                is_utils_package: false,
            });
        }),
    ];
    for (const webview of globSync(
        `${root}${root.endsWith("/") ? "" : "/"}packages/webviews/*/`,
    )) {
        entities.push(
            new RepoEntity({
                path: webview,
                type: "package",
                is_webview: true,
                is_utils_package: false,
            }),
        );
    }
    const packages = [
        ...globSync(
            `${root}${root.endsWith("/") ? "" : "/"}packages/*/`,
        ),
    ];
    console.log("packages: ", packages);
    return entities;
};

getRepoEntities();
